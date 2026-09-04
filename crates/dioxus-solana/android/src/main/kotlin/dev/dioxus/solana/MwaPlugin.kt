package dev.dioxus.solana

import android.app.Activity
import android.app.Application
import android.content.ContentProvider
import android.content.ContentValues
import android.database.Cursor
import android.net.Uri
import android.os.Bundle
import android.util.Base64
import androidx.activity.ComponentActivity
import com.solana.mobilewalletadapter.clientlib.ActivityResultSender
import com.solana.mobilewalletadapter.clientlib.ConnectionIdentity
import com.solana.mobilewalletadapter.clientlib.MobileWalletAdapter
import com.solana.mobilewalletadapter.clientlib.RpcCluster
import com.solana.mobilewalletadapter.clientlib.TransactionResult
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

/**
 * JNI entry for native Android MWA. Session state lives on [Host] so a new
 * JNI wrapper per call still sees the authorized adapter.
 *
 * [ActivityResultSender] must be constructed in [Activity.onCreate], before
 * STARTED. [MwaInitProvider] registers for that.
 */
class MwaPlugin(private val activity: Activity) {
    fun connect(
        requestId: Long,
        silent: String,
        cluster: String,
        name: String,
        uri: String,
        icon: String,
    ) {
        scope.launch {
            try {
                val adapter = adapter(name, uri, icon, cluster)
                if (silent == "1" && adapter.authToken == null) {
                    onResult(requestId, "disconnected", "", "")
                    return@launch
                }
                when (val result = adapter.connect(sender())) {
                    is TransactionResult.Success -> {
                        val pk = result.authResult.accounts.first().publicKey
                        authPubkey = pk
                        onResult(requestId, "ok", b64(pk), "")
                    }
                    is TransactionResult.NoWalletFound ->
                        onResult(requestId, "not_installed", "", result.message)
                    is TransactionResult.Failure -> fail(requestId, result)
                }
            } catch (e: Exception) {
                onResult(requestId, "err", "", e.message ?: "mwa connect failed")
            }
        }
    }

    fun signMessage(requestId: Long, messageB64: String) {
        scope.launch {
            try {
                val adapter = heldAdapter ?: run {
                    onResult(requestId, "disconnected", "", "")
                    return@launch
                }
                val message = Base64.decode(messageB64, Base64.DEFAULT)
                val pk = authPubkey ?: run {
                    onResult(requestId, "disconnected", "", "")
                    return@launch
                }
                when (
                    val result = adapter.transact(sender()) { _ ->
                        signMessagesDetached(arrayOf(message), arrayOf(pk))
                    }
                ) {
                    is TransactionResult.Success -> {
                        val sig = result.payload.messages.first().signatures.first()
                        onResult(requestId, "ok", b64(sig), "")
                    }
                    is TransactionResult.NoWalletFound ->
                        onResult(requestId, "not_installed", "", result.message)
                    is TransactionResult.Failure -> fail(requestId, result)
                }
            } catch (e: Exception) {
                onResult(requestId, "err", "", e.message ?: "mwa sign failed")
            }
        }
    }

    fun signTransaction(requestId: Long, txB64: String) {
        signTx(requestId, txB64, send = false)
    }

    fun signAndSend(requestId: Long, txB64: String) {
        signTx(requestId, txB64, send = true)
    }

    fun disconnect(requestId: Long) {
        scope.launch {
            try {
                val adapter = heldAdapter
                if (adapter == null) {
                    onResult(requestId, "ok", "", "")
                    return@launch
                }
                when (val result = adapter.disconnect(sender())) {
                    is TransactionResult.Success -> {
                        authPubkey = null
                        onResult(requestId, "ok", "", "")
                    }
                    is TransactionResult.NoWalletFound ->
                        onResult(requestId, "not_installed", "", result.message)
                    is TransactionResult.Failure -> fail(requestId, result)
                }
            } catch (e: Exception) {
                onResult(requestId, "err", "", e.message ?: "mwa disconnect failed")
            }
        }
    }

    private fun signTx(requestId: Long, txB64: String, send: Boolean) {
        scope.launch {
            try {
                val adapter = heldAdapter ?: run {
                    onResult(requestId, "disconnected", "", "")
                    return@launch
                }
                val tx = Base64.decode(txB64, Base64.DEFAULT)
                if (send) {
                    when (
                        val result = adapter.transact(sender()) { _ ->
                            signAndSendTransactions(arrayOf(tx))
                        }
                    ) {
                        is TransactionResult.Success -> {
                            val sig = result.payload.signatures.first()
                            onResult(requestId, "ok", b64(sig), "")
                        }
                        is TransactionResult.NoWalletFound ->
                            onResult(requestId, "not_installed", "", result.message)
                        is TransactionResult.Failure -> fail(requestId, result)
                    }
                } else {
                    when (
                        val result = adapter.transact(sender()) { _ ->
                            signTransactions(arrayOf(tx))
                        }
                    ) {
                        is TransactionResult.Success -> {
                            val signed = result.payload.signedPayloads.first()
                            onResult(requestId, "ok", b64(signed), "")
                        }
                        is TransactionResult.NoWalletFound ->
                            onResult(requestId, "not_installed", "", result.message)
                        is TransactionResult.Failure -> fail(requestId, result)
                    }
                }
            } catch (e: Exception) {
                onResult(requestId, "err", "", e.message ?: "mwa tx failed")
            }
        }
    }

    private fun sender(): ActivityResultSender {
        Host.sender?.let { return it }
        val host = activity as? ComponentActivity
            ?: throw IllegalStateException("MWA needs a ComponentActivity")
        return Host.attach(host)
    }

    companion object {
        private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main)
        private var heldAdapter: MobileWalletAdapter? = null
        private var authPubkey: ByteArray? = null

        @JvmStatic
        external fun onResult(requestId: Long, status: String, payload: String, error: String)

        private fun adapter(
            name: String,
            uri: String,
            icon: String,
            cluster: String,
        ): MobileWalletAdapter {
            heldAdapter?.let {
                applyCluster(it, cluster)
                return it
            }
            val identityUri = Uri.parse(uri.ifBlank { "https://localhost" })
            val created = MobileWalletAdapter(
                connectionIdentity = ConnectionIdentity(
                    identityUri = identityUri,
                    iconUri = Uri.parse(icon.ifBlank { "favicon.ico" }),
                    identityName = name.ifBlank { "Dioxus" },
                )
            )
            applyCluster(created, cluster)
            heldAdapter = created
            return created
        }

        private fun applyCluster(adapter: MobileWalletAdapter, cluster: String) {
            adapter.rpcCluster = when (cluster) {
                "mainnet-beta" -> RpcCluster.MainnetBeta
                "testnet" -> RpcCluster.Testnet
                else -> RpcCluster.Devnet
            }
        }

        private fun fail(requestId: Long, result: TransactionResult.Failure<*>) {
            val msg = result.message.ifBlank { result.e.message ?: "mwa failed" }
            val status = if (
                msg.contains("did not authorize", ignoreCase = true) ||
                msg.contains("interrupted", ignoreCase = true) ||
                msg.contains("cancelled", ignoreCase = true)
            ) {
                "rejected"
            } else {
                "err"
            }
            onResult(requestId, status, "", msg)
        }

        private fun b64(bytes: ByteArray): String =
            Base64.encodeToString(bytes, Base64.NO_WRAP)
    }
}

internal object Host {
    @Volatile
    var sender: ActivityResultSender? = null
        private set

    @Volatile
    private var activity: ComponentActivity? = null

    fun attach(activity: Activity): ActivityResultSender {
        val host = activity as? ComponentActivity
            ?: throw IllegalStateException("MWA needs a ComponentActivity")
        sender?.let { existing ->
            if (this.activity === host) {
                return existing
            }
        }
        val created = ActivityResultSender(host)
        this.activity = host
        sender = created
        return created
    }

    fun detach(activity: Activity) {
        if (this.activity === activity) {
            sender = null
            this.activity = null
        }
    }
}

/**
 * Runs before any Activity. Registers [ActivityResultSender] in
 * [Application.ActivityLifecycleCallbacks.onActivityCreated].
 */
class MwaInitProvider : ContentProvider() {
    override fun onCreate(): Boolean {
        val app = context?.applicationContext as? Application ?: return false
        app.registerActivityLifecycleCallbacks(object : Application.ActivityLifecycleCallbacks {
            override fun onActivityCreated(activity: Activity, savedInstanceState: Bundle?) {
                if (activity is ComponentActivity) {
                    Host.attach(activity)
                }
            }

            override fun onActivityDestroyed(activity: Activity) {
                Host.detach(activity)
            }

            override fun onActivityStarted(activity: Activity) {}
            override fun onActivityResumed(activity: Activity) {}
            override fun onActivityPaused(activity: Activity) {}
            override fun onActivityStopped(activity: Activity) {}
            override fun onActivitySaveInstanceState(activity: Activity, outState: Bundle) {}
        })
        return true
    }

    override fun query(
        uri: Uri,
        projection: Array<out String>?,
        selection: String?,
        selectionArgs: Array<out String>?,
        sortOrder: String?,
    ): Cursor? = null

    override fun getType(uri: Uri): String? = null
    override fun insert(uri: Uri, values: ContentValues?): Uri? = null
    override fun delete(uri: Uri, selection: String?, selectionArgs: Array<out String>?): Int = 0
    override fun update(
        uri: Uri,
        values: ContentValues?,
        selection: String?,
        selectionArgs: Array<out String>?,
    ): Int = 0
}

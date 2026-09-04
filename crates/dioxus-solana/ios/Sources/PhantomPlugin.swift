import Foundation
import ObjectiveC.runtime
import UIKit

@_silgen_name("dioxus_solana_phantom_on_result")
func dioxus_solana_phantom_on_result(
    _ id: Int64,
    _ status: UnsafePointer<CChar>?,
    _ payload: UnsafePointer<CChar>?,
    _ error: UnsafePointer<CChar>?
)

/// Opens Phantom universal links and returns the custom-scheme callback to Rust.
@objc(PhantomPlugin)
public class PhantomPlugin: NSObject {
    private static let scheme = "dioxussolana"
    private static var pendingId: Int64 = 0
    private static let lock = NSLock()
    private static var hooked = false

    @objc override public init() {
        super.init()
        PhantomPlugin.install()
    }

    @objc public func openUrl(_ request: String) -> String {
        let parts = request.split(separator: "\n", maxSplits: 1)
        guard parts.count == 2, let requestId = Int64(parts[0]) else {
            PhantomPlugin.finish(0, status: "err", payload: "", error: "bad phantom request")
            return ""
        }
        let url = String(parts[1])
        PhantomPlugin.lock.lock()
        PhantomPlugin.pendingId = requestId
        PhantomPlugin.lock.unlock()

        guard let parsed = URL(string: url) else {
            PhantomPlugin.finish(requestId, status: "err", payload: "", error: "bad phantom url")
            return ""
        }

        DispatchQueue.main.async {
            guard let probe = URL(string: "phantom://"),
                  UIApplication.shared.canOpenURL(probe)
            else {
                PhantomPlugin.finish(
                    requestId,
                    status: "not_installed",
                    payload: "",
                    error: "Phantom is not installed"
                )
                return
            }
            UIApplication.shared.open(parsed, options: [:]) { ok in
                if !ok {
                    PhantomPlugin.finish(
                        requestId,
                        status: "not_installed",
                        payload: "",
                        error: "cannot open Phantom"
                    )
                }
            }
        }
        return ""
    }

    @discardableResult
    static func handle(_ url: URL) -> Bool {
        guard url.scheme?.lowercased() == scheme else { return false }
        lock.lock()
        let id = pendingId
        pendingId = 0
        lock.unlock()
        guard id != 0 else { return false }
        finish(id, status: "ok", payload: url.absoluteString, error: "")
        return true
    }

    private static func finish(_ id: Int64, status: String, payload: String, error: String) {
        status.withCString { statusPtr in
            payload.withCString { payloadPtr in
                error.withCString { errorPtr in
                    dioxus_solana_phantom_on_result(id, statusPtr, payloadPtr, errorPtr)
                }
            }
        }
    }

    private static var hookedKeys = Set<String>()
    private static var swappedKeys = Set<String>()

    private static func hookKey(cls: AnyClass, sel: Selector) -> String {
        "\(NSStringFromClass(cls))|\(NSStringFromSelector(sel))"
    }

    private static func install() {
        DispatchQueue.main.async {
            guard !hooked else { return }
            hooked = true
            hookAppDelegate()
            hookScenes()
            NotificationCenter.default.addObserver(
                forName: UIScene.willConnectNotification,
                object: nil,
                queue: .main
            ) { _ in
                hookScenes()
            }
        }
    }

    private static func hookSelector(cls: AnyClass, original: Selector, added: Selector) {
        let key = hookKey(cls: cls, sel: original)
        guard !hookedKeys.contains(key) else { return }
        hookedKeys.insert(key)
        guard let method = class_getInstanceMethod(PhantomPlugin.self, added) else { return }
        let impl = method_getImplementation(method)
        let types = method_getTypeEncoding(method)
        if class_getInstanceMethod(cls, original) == nil {
            _ = class_addMethod(cls, original, impl, types)
            return
        }
        _ = class_addMethod(cls, added, impl, types)
        guard let origM = class_getInstanceMethod(cls, original),
              let addM = class_getInstanceMethod(cls, added)
        else { return }
        method_exchangeImplementations(origM, addM)
        swappedKeys.insert(key)
    }

    private static func hookAppDelegate() {
        guard let delegate = UIApplication.shared.delegate else { return }
        guard let cls: AnyClass = object_getClass(delegate) else { return }
        hookSelector(
            cls: cls,
            original: NSSelectorFromString("application:openURL:options:"),
            added: #selector(PhantomPlugin.solana_application(_:open:options:))
        )
    }

    private static func hookScenes() {
        for scene in UIApplication.shared.connectedScenes {
            guard let delegate = scene.delegate else { continue }
            guard let cls: AnyClass = object_getClass(delegate) else { continue }
            hookSelector(
                cls: cls,
                original: NSSelectorFromString("scene:openURLContexts:"),
                added: #selector(PhantomPlugin.solana_scene(_:openURLContexts:))
            )
        }
    }

    @objc func solana_application(
        _ app: UIApplication,
        open url: URL,
        options: [UIApplication.OpenURLOptionsKey: Any] = [:]
    ) -> Bool {
        if PhantomPlugin.handle(url) {
            return true
        }
        guard let cls = object_getClass(self) else { return false }
        let key = PhantomPlugin.hookKey(
            cls: cls,
            sel: NSSelectorFromString("application:openURL:options:")
        )
        if PhantomPlugin.swappedKeys.contains(key) {
            return solana_application(app, open: url, options: options)
        }
        return false
    }

    @objc func solana_scene(_ scene: UIScene, openURLContexts URLContexts: Set<UIOpenURLContext>) {
        for ctx in URLContexts {
            _ = PhantomPlugin.handle(ctx.url)
        }
        guard let cls = object_getClass(self) else { return }
        let key = PhantomPlugin.hookKey(
            cls: cls,
            sel: NSSelectorFromString("scene:openURLContexts:")
        )
        if PhantomPlugin.swappedKeys.contains(key) {
            solana_scene(scene, openURLContexts: URLContexts)
        }
    }
}

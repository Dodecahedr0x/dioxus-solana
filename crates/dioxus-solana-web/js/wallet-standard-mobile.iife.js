/* @solana-mobile/wallet-standard-mobile@0.5.2 — regenerate with js/build.sh */
var SolanaMobileWalletStandard=(()=>{var Ea=Object.create;var Ne=Object.defineProperty;var ha=Object.getOwnPropertyDescriptor;var Aa=Object.getOwnPropertyNames;var Oa=Object.getPrototypeOf,Na=Object.prototype.hasOwnProperty;var S=(t,e)=>()=>{try{return e||t((e={exports:{}}).exports,e),e.exports}catch(n){throw e=0,n}},Sa=(t,e)=>{for(var n in e)Ne(t,n,{get:e[n],enumerable:!0})},gt=(t,e,n,a)=>{if(e&&typeof e=="object"||typeof e=="function")for(let r of Aa(e))!Na.call(t,r)&&r!==n&&Ne(t,r,{get:()=>e[r],enumerable:!(a=ha(e,r))||a.enumerable});return t};var ma=(t,e,n)=>(n=t!=null?Ea(Oa(t)):{},gt(e||!t||!t.__esModule?Ne(n,"default",{value:t,enumerable:!0}):n,t)),pa=t=>gt(Ne({},"__esModule",{value:!0}),t);var nn=S((Ou,tn)=>{tn.exports=function(){return typeof Promise=="function"&&Promise.prototype&&Promise.prototype.then}});var $=S(K=>{var Je,Rd=[0,26,44,70,100,134,172,196,242,292,346,404,466,532,581,655,733,815,901,991,1085,1156,1258,1364,1474,1588,1706,1828,1921,2051,2185,2323,2465,2611,2761,2876,3034,3196,3362,3532,3706];K.getSymbolSize=function(e){if(!e)throw new Error('"version" cannot be null or undefined');if(e<1||e>40)throw new Error('"version" should be in range from 1 to 40');return e*4+17};K.getSymbolTotalCodewords=function(e){return Rd[e]};K.getBCHDigit=function(t){let e=0;for(;t!==0;)e++,t>>>=1;return e};K.setToSJISFunction=function(e){if(typeof e!="function")throw new Error('"toSJISFunc" is not a valid function.');Je=e};K.isKanjiModeEnabled=function(){return typeof Je<"u"};K.toSJIS=function(e){return Je(e)}});var be=S(U=>{U.L={bit:1};U.M={bit:0};U.Q={bit:3};U.H={bit:2};function Ed(t){if(typeof t!="string")throw new Error("Param is not a string");switch(t.toLowerCase()){case"l":case"low":return U.L;case"m":case"medium":return U.M;case"q":case"quartile":return U.Q;case"h":case"high":return U.H;default:throw new Error("Unknown EC Level: "+t)}}U.isValid=function(e){return e&&typeof e.bit<"u"&&e.bit>=0&&e.bit<4};U.from=function(e,n){if(U.isValid(e))return e;try{return Ed(e)}catch{return n}}});var on=S((mu,rn)=>{function an(){this.buffer=[],this.length=0}an.prototype={get:function(t){let e=Math.floor(t/8);return(this.buffer[e]>>>7-t%8&1)===1},put:function(t,e){for(let n=0;n<e;n++)this.putBit((t>>>e-n-1&1)===1)},getLengthInBits:function(){return this.length},putBit:function(t){let e=Math.floor(this.length/8);this.buffer.length<=e&&this.buffer.push(0),t&&(this.buffer[e]|=128>>>this.length%8),this.length++}};rn.exports=an});var cn=S((pu,sn)=>{function oe(t){if(!t||t<1)throw new Error("BitMatrix size must be defined and greater than 0");this.size=t,this.data=new Uint8Array(t*t),this.reservedBit=new Uint8Array(t*t)}oe.prototype.set=function(t,e,n,a){let r=t*this.size+e;this.data[r]=n,a&&(this.reservedBit[r]=!0)};oe.prototype.get=function(t,e){return this.data[t*this.size+e]};oe.prototype.xor=function(t,e,n){this.data[t*this.size+e]^=n};oe.prototype.isReserved=function(t,e){return this.reservedBit[t*this.size+e]};sn.exports=oe});var ln=S(ye=>{var hd=$().getSymbolSize;ye.getRowColCoords=function(e){if(e===1)return[];let n=Math.floor(e/7)+2,a=hd(e),r=a===145?26:Math.ceil((a-13)/(2*n-2))*2,o=[a-7];for(let i=1;i<n-1;i++)o[i]=o[i-1]-r;return o.push(6),o.reverse()};ye.getPositions=function(e){let n=[],a=ye.getRowColCoords(e),r=a.length;for(let o=0;o<r;o++)for(let i=0;i<r;i++)o===0&&i===0||o===0&&i===r-1||o===r-1&&i===0||n.push([a[o],a[i]]);return n}});var un=S(_n=>{var Ad=$().getSymbolSize,dn=7;_n.getPositions=function(e){let n=Ad(e);return[[0,0],[n-dn,0],[0,n-dn]]}});var Rn=S(T=>{T.Patterns={PATTERN000:0,PATTERN001:1,PATTERN010:2,PATTERN011:3,PATTERN100:4,PATTERN101:5,PATTERN110:6,PATTERN111:7};var W={N1:3,N2:3,N3:40,N4:10};T.isValid=function(e){return e!=null&&e!==""&&!isNaN(e)&&e>=0&&e<=7};T.from=function(e){return T.isValid(e)?parseInt(e,10):void 0};T.getPenaltyN1=function(e){let n=e.size,a=0,r=0,o=0,i=null,s=null;for(let c=0;c<n;c++){r=o=0,i=s=null;for(let l=0;l<n;l++){let d=e.get(c,l);d===i?r++:(r>=5&&(a+=W.N1+(r-5)),i=d,r=1),d=e.get(l,c),d===s?o++:(o>=5&&(a+=W.N1+(o-5)),s=d,o=1)}r>=5&&(a+=W.N1+(r-5)),o>=5&&(a+=W.N1+(o-5))}return a};T.getPenaltyN2=function(e){let n=e.size,a=0;for(let r=0;r<n-1;r++)for(let o=0;o<n-1;o++){let i=e.get(r,o)+e.get(r,o+1)+e.get(r+1,o)+e.get(r+1,o+1);(i===4||i===0)&&a++}return a*W.N2};T.getPenaltyN3=function(e){let n=e.size,a=0,r=0,o=0;for(let i=0;i<n;i++){r=o=0;for(let s=0;s<n;s++)r=r<<1&2047|e.get(i,s),s>=10&&(r===1488||r===93)&&a++,o=o<<1&2047|e.get(s,i),s>=10&&(o===1488||o===93)&&a++}return a*W.N3};T.getPenaltyN4=function(e){let n=0,a=e.data.length;for(let o=0;o<a;o++)n+=e.data[o];return Math.abs(Math.ceil(n*100/a/5)-10)*W.N4};function Od(t,e,n){switch(t){case T.Patterns.PATTERN000:return(e+n)%2===0;case T.Patterns.PATTERN001:return e%2===0;case T.Patterns.PATTERN010:return n%3===0;case T.Patterns.PATTERN011:return(e+n)%3===0;case T.Patterns.PATTERN100:return(Math.floor(e/2)+Math.floor(n/3))%2===0;case T.Patterns.PATTERN101:return e*n%2+e*n%3===0;case T.Patterns.PATTERN110:return(e*n%2+e*n%3)%2===0;case T.Patterns.PATTERN111:return(e*n%3+(e+n)%2)%2===0;default:throw new Error("bad maskPattern:"+t)}}T.applyMask=function(e,n){let a=n.size;for(let r=0;r<a;r++)for(let o=0;o<a;o++)n.isReserved(o,r)||n.xor(o,r,Od(e,o,r))};T.getBestMask=function(e,n){let a=Object.keys(T.Patterns).length,r=0,o=1/0;for(let i=0;i<a;i++){n(i),T.applyMask(i,e);let s=T.getPenaltyN1(e)+T.getPenaltyN2(e)+T.getPenaltyN3(e)+T.getPenaltyN4(e);T.applyMask(i,e),s<o&&(o=s,r=i)}return r}});var Qe=S(Ze=>{var k=be(),ve=[1,1,1,1,1,1,1,1,1,1,2,2,1,2,2,4,1,2,4,4,2,4,4,4,2,4,6,5,2,4,6,6,2,5,8,8,4,5,8,8,4,5,8,11,4,8,10,11,4,9,12,16,4,9,16,16,6,10,12,18,6,10,17,16,6,11,16,19,6,13,18,21,7,14,21,25,8,16,20,25,8,17,23,25,9,17,23,34,9,18,25,30,10,20,27,32,12,21,29,35,12,23,34,37,12,25,34,40,13,26,35,42,14,28,38,45,15,29,40,48,16,31,43,51,17,33,45,54,18,35,48,57,19,37,51,60,19,38,53,63,20,40,56,66,21,43,59,70,22,45,62,74,24,47,65,77,25,49,68,81],De=[7,10,13,17,10,16,22,28,15,26,36,44,20,36,52,64,26,48,72,88,36,64,96,112,40,72,108,130,48,88,132,156,60,110,160,192,72,130,192,224,80,150,224,264,96,176,260,308,104,198,288,352,120,216,320,384,132,240,360,432,144,280,408,480,168,308,448,532,180,338,504,588,196,364,546,650,224,416,600,700,224,442,644,750,252,476,690,816,270,504,750,900,300,560,810,960,312,588,870,1050,336,644,952,1110,360,700,1020,1200,390,728,1050,1260,420,784,1140,1350,450,812,1200,1440,480,868,1290,1530,510,924,1350,1620,540,980,1440,1710,570,1036,1530,1800,570,1064,1590,1890,600,1120,1680,1980,630,1204,1770,2100,660,1260,1860,2220,720,1316,1950,2310,750,1372,2040,2430];Ze.getBlocksCount=function(e,n){switch(n){case k.L:return ve[(e-1)*4+0];case k.M:return ve[(e-1)*4+1];case k.Q:return ve[(e-1)*4+2];case k.H:return ve[(e-1)*4+3];default:return}};Ze.getTotalCodewordsCount=function(e,n){switch(n){case k.L:return De[(e-1)*4+0];case k.M:return De[(e-1)*4+1];case k.Q:return De[(e-1)*4+2];case k.H:return De[(e-1)*4+3];default:return}}});var En=S(Me=>{var ie=new Uint8Array(512),xe=new Uint8Array(256);(function(){let e=1;for(let n=0;n<255;n++)ie[n]=e,xe[e]=n,e<<=1,e&256&&(e^=285);for(let n=255;n<512;n++)ie[n]=ie[n-255]})();Me.log=function(e){if(e<1)throw new Error("log("+e+")");return xe[e]};Me.exp=function(e){return ie[e]};Me.mul=function(e,n){return e===0||n===0?0:ie[xe[e]+xe[n]]}});var hn=S(se=>{var et=En();se.mul=function(e,n){let a=new Uint8Array(e.length+n.length-1);for(let r=0;r<e.length;r++)for(let o=0;o<n.length;o++)a[r+o]^=et.mul(e[r],n[o]);return a};se.mod=function(e,n){let a=new Uint8Array(e);for(;a.length-n.length>=0;){let r=a[0];for(let i=0;i<n.length;i++)a[i]^=et.mul(n[i],r);let o=0;for(;o<a.length&&a[o]===0;)o++;a=a.slice(o)}return a};se.generateECPolynomial=function(e){let n=new Uint8Array([1]);for(let a=0;a<e;a++)n=se.mul(n,new Uint8Array([1,et.exp(a)]));return n}});var Nn=S((Lu,On)=>{var An=hn();function tt(t){this.genPoly=void 0,this.degree=t,this.degree&&this.initialize(this.degree)}tt.prototype.initialize=function(e){this.degree=e,this.genPoly=An.generateECPolynomial(this.degree)};tt.prototype.encode=function(e){if(!this.genPoly)throw new Error("Encoder not initialized");let n=new Uint8Array(e.length+this.degree);n.set(e);let a=An.mod(n,this.genPoly),r=this.degree-a.length;if(r>0){let o=new Uint8Array(this.degree);return o.set(a,r),o}return a};On.exports=tt});var nt=S(Sn=>{Sn.isValid=function(e){return!isNaN(e)&&e>=1&&e<=40}});var at=S(P=>{var mn="[0-9]+",Nd="[A-Z $%*+\\-./:]+",ce="(?:[u3000-u303F]|[u3040-u309F]|[u30A0-u30FF]|[uFF00-uFFEF]|[u4E00-u9FAF]|[u2605-u2606]|[u2190-u2195]|u203B|[u2010u2015u2018u2019u2025u2026u201Cu201Du2225u2260]|[u0391-u0451]|[u00A7u00A8u00B1u00B4u00D7u00F7])+";ce=ce.replace(/u/g,"\\u");var Sd="(?:(?![A-Z0-9 $%*+\\-./:]|"+ce+`)(?:.|[\r
]))+`;P.KANJI=new RegExp(ce,"g");P.BYTE_KANJI=new RegExp("[^A-Z0-9 $%*+\\-./:]+","g");P.BYTE=new RegExp(Sd,"g");P.NUMERIC=new RegExp(mn,"g");P.ALPHANUMERIC=new RegExp(Nd,"g");var md=new RegExp("^"+ce+"$"),pd=new RegExp("^"+mn+"$"),fd=new RegExp("^[A-Z0-9 $%*+\\-./:]+$");P.testKanji=function(e){return md.test(e)};P.testNumeric=function(e){return pd.test(e)};P.testAlphanumeric=function(e){return fd.test(e)}});var V=S(y=>{var gd=nt(),rt=at();y.NUMERIC={id:"Numeric",bit:1,ccBits:[10,12,14]};y.ALPHANUMERIC={id:"Alphanumeric",bit:2,ccBits:[9,11,13]};y.BYTE={id:"Byte",bit:4,ccBits:[8,16,16]};y.KANJI={id:"Kanji",bit:8,ccBits:[8,10,12]};y.MIXED={bit:-1};y.getCharCountIndicator=function(e,n){if(!e.ccBits)throw new Error("Invalid mode: "+e);if(!gd.isValid(n))throw new Error("Invalid version: "+n);return n>=1&&n<10?e.ccBits[0]:n<27?e.ccBits[1]:e.ccBits[2]};y.getBestModeForData=function(e){return rt.testNumeric(e)?y.NUMERIC:rt.testAlphanumeric(e)?y.ALPHANUMERIC:rt.testKanji(e)?y.KANJI:y.BYTE};y.toString=function(e){if(e&&e.id)return e.id;throw new Error("Invalid mode")};y.isValid=function(e){return e&&e.bit&&e.ccBits};function Td(t){if(typeof t!="string")throw new Error("Param is not a string");switch(t.toLowerCase()){case"numeric":return y.NUMERIC;case"alphanumeric":return y.ALPHANUMERIC;case"kanji":return y.KANJI;case"byte":return y.BYTE;default:throw new Error("Unknown mode: "+t)}}y.from=function(e,n){if(y.isValid(e))return e;try{return Td(e)}catch{return n}}});var In=S(q=>{var Ue=$(),Id=Qe(),pn=be(),z=V(),ot=nt(),gn=7973,fn=Ue.getBCHDigit(gn);function Cd(t,e,n){for(let a=1;a<=40;a++)if(e<=q.getCapacity(a,n,t))return a}function Tn(t,e){return z.getCharCountIndicator(t,e)+4}function wd(t,e){let n=0;return t.forEach(function(a){let r=Tn(a.mode,e);n+=r+a.getBitsLength()}),n}function Ld(t,e){for(let n=1;n<=40;n++)if(wd(t,n)<=q.getCapacity(n,e,z.MIXED))return n}q.from=function(e,n){return ot.isValid(e)?parseInt(e,10):n};q.getCapacity=function(e,n,a){if(!ot.isValid(e))throw new Error("Invalid QR Code version");typeof a>"u"&&(a=z.BYTE);let r=Ue.getSymbolTotalCodewords(e),o=Id.getTotalCodewordsCount(e,n),i=(r-o)*8;if(a===z.MIXED)return i;let s=i-Tn(a,e);switch(a){case z.NUMERIC:return Math.floor(s/10*3);case z.ALPHANUMERIC:return Math.floor(s/11*2);case z.KANJI:return Math.floor(s/13);case z.BYTE:default:return Math.floor(s/8)}};q.getBestVersionForData=function(e,n){let a,r=pn.from(n,pn.M);if(Array.isArray(e)){if(e.length>1)return Ld(e,r);if(e.length===0)return 1;a=e[0]}else a=e;return Cd(a.mode,a.getLength(),r)};q.getEncodedBits=function(e){if(!ot.isValid(e)||e<7)throw new Error("Invalid QR Code version");let n=e<<12;for(;Ue.getBCHDigit(n)-fn>=0;)n^=gn<<Ue.getBCHDigit(n)-fn;return e<<12|n}});var bn=S(Ln=>{var it=$(),wn=1335,bd=21522,Cn=it.getBCHDigit(wn);Ln.getEncodedBits=function(e,n){let a=e.bit<<3|n,r=a<<10;for(;it.getBCHDigit(r)-Cn>=0;)r^=wn<<it.getBCHDigit(r)-Cn;return(a<<10|r)^bd}});var vn=S((Mu,yn)=>{var yd=V();function J(t){this.mode=yd.NUMERIC,this.data=t.toString()}J.getBitsLength=function(e){return 10*Math.floor(e/3)+(e%3?e%3*3+1:0)};J.prototype.getLength=function(){return this.data.length};J.prototype.getBitsLength=function(){return J.getBitsLength(this.data.length)};J.prototype.write=function(e){let n,a,r;for(n=0;n+3<=this.data.length;n+=3)a=this.data.substr(n,3),r=parseInt(a,10),e.put(r,10);let o=this.data.length-n;o>0&&(a=this.data.substr(n),r=parseInt(a,10),e.put(r,o*3+1))};yn.exports=J});var xn=S((Uu,Dn)=>{var vd=V(),st=["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"," ","$","%","*","+","-",".","/",":"];function Z(t){this.mode=vd.ALPHANUMERIC,this.data=t}Z.getBitsLength=function(e){return 11*Math.floor(e/2)+6*(e%2)};Z.prototype.getLength=function(){return this.data.length};Z.prototype.getBitsLength=function(){return Z.getBitsLength(this.data.length)};Z.prototype.write=function(e){let n;for(n=0;n+2<=this.data.length;n+=2){let a=st.indexOf(this.data[n])*45;a+=st.indexOf(this.data[n+1]),e.put(a,11)}this.data.length%2&&e.put(st.indexOf(this.data[n]),6)};Dn.exports=Z});var Un=S((Pu,Mn)=>{var Dd=V();function Q(t){this.mode=Dd.BYTE,typeof t=="string"?this.data=new TextEncoder().encode(t):this.data=new Uint8Array(t)}Q.getBitsLength=function(e){return e*8};Q.prototype.getLength=function(){return this.data.length};Q.prototype.getBitsLength=function(){return Q.getBitsLength(this.data.length)};Q.prototype.write=function(t){for(let e=0,n=this.data.length;e<n;e++)t.put(this.data[e],8)};Mn.exports=Q});var Bn=S((Bu,Pn)=>{var xd=V(),Md=$();function ee(t){this.mode=xd.KANJI,this.data=t}ee.getBitsLength=function(e){return e*13};ee.prototype.getLength=function(){return this.data.length};ee.prototype.getBitsLength=function(){return ee.getBitsLength(this.data.length)};ee.prototype.write=function(t){let e;for(e=0;e<this.data.length;e++){let n=Md.toSJIS(this.data[e]);if(n>=33088&&n<=40956)n-=33088;else if(n>=57408&&n<=60351)n-=49472;else throw new Error("Invalid SJIS character: "+this.data[e]+`
Make sure your charset is UTF-8`);n=(n>>>8&255)*192+(n&255),t.put(n,13)}};Pn.exports=ee});var Fn=S((Fu,ct)=>{"use strict";var le={single_source_shortest_paths:function(t,e,n){var a={},r={};r[e]=0;var o=le.PriorityQueue.make();o.push(e,0);for(var i,s,c,l,d,R,_,O,f;!o.empty();){i=o.pop(),s=i.value,l=i.cost,d=t[s]||{};for(c in d)d.hasOwnProperty(c)&&(R=d[c],_=l+R,O=r[c],f=typeof r[c]>"u",(f||O>_)&&(r[c]=_,o.push(c,_),a[c]=s))}if(typeof n<"u"&&typeof r[n]>"u"){var A=["Could not find a path from ",e," to ",n,"."].join("");throw new Error(A)}return a},extract_shortest_path_from_predecessor_list:function(t,e){for(var n=[],a=e,r;a;)n.push(a),r=t[a],a=t[a];return n.reverse(),n},find_path:function(t,e,n){var a=le.single_source_shortest_paths(t,e,n);return le.extract_shortest_path_from_predecessor_list(a,n)},PriorityQueue:{make:function(t){var e=le.PriorityQueue,n={},a;t=t||{};for(a in e)e.hasOwnProperty(a)&&(n[a]=e[a]);return n.queue=[],n.sorter=t.sorter||e.default_sorter,n},default_sorter:function(t,e){return t.cost-e.cost},push:function(t,e){var n={value:t,cost:e};this.queue.push(n),this.queue.sort(this.sorter)},pop:function(){return this.queue.shift()},empty:function(){return this.queue.length===0}}};typeof ct<"u"&&(ct.exports=le)});var Wn=S(te=>{var N=V(),Vn=vn(),zn=xn(),Gn=Un(),Hn=Bn(),de=at(),Pe=$(),Ud=Fn();function $n(t){return unescape(encodeURIComponent(t)).length}function _e(t,e,n){let a=[],r;for(;(r=t.exec(n))!==null;)a.push({data:r[0],index:r.index,mode:e,length:r[0].length});return a}function Kn(t){let e=_e(de.NUMERIC,N.NUMERIC,t),n=_e(de.ALPHANUMERIC,N.ALPHANUMERIC,t),a,r;return Pe.isKanjiModeEnabled()?(a=_e(de.BYTE,N.BYTE,t),r=_e(de.KANJI,N.KANJI,t)):(a=_e(de.BYTE_KANJI,N.BYTE,t),r=[]),e.concat(n,a,r).sort(function(i,s){return i.index-s.index}).map(function(i){return{data:i.data,mode:i.mode,length:i.length}})}function lt(t,e){switch(e){case N.NUMERIC:return Vn.getBitsLength(t);case N.ALPHANUMERIC:return zn.getBitsLength(t);case N.KANJI:return Hn.getBitsLength(t);case N.BYTE:return Gn.getBitsLength(t)}}function Pd(t){return t.reduce(function(e,n){let a=e.length-1>=0?e[e.length-1]:null;return a&&a.mode===n.mode?(e[e.length-1].data+=n.data,e):(e.push(n),e)},[])}function Bd(t){let e=[];for(let n=0;n<t.length;n++){let a=t[n];switch(a.mode){case N.NUMERIC:e.push([a,{data:a.data,mode:N.ALPHANUMERIC,length:a.length},{data:a.data,mode:N.BYTE,length:a.length}]);break;case N.ALPHANUMERIC:e.push([a,{data:a.data,mode:N.BYTE,length:a.length}]);break;case N.KANJI:e.push([a,{data:a.data,mode:N.BYTE,length:$n(a.data)}]);break;case N.BYTE:e.push([{data:a.data,mode:N.BYTE,length:$n(a.data)}])}}return e}function Fd(t,e){let n={},a={start:{}},r=["start"];for(let o=0;o<t.length;o++){let i=t[o],s=[];for(let c=0;c<i.length;c++){let l=i[c],d=""+o+c;s.push(d),n[d]={node:l,lastCount:0},a[d]={};for(let R=0;R<r.length;R++){let _=r[R];n[_]&&n[_].node.mode===l.mode?(a[_][d]=lt(n[_].lastCount+l.length,l.mode)-lt(n[_].lastCount,l.mode),n[_].lastCount+=l.length):(n[_]&&(n[_].lastCount=l.length),a[_][d]=lt(l.length,l.mode)+4+N.getCharCountIndicator(l.mode,e))}}r=s}for(let o=0;o<r.length;o++)a[r[o]].end=0;return{map:a,table:n}}function kn(t,e){let n,a=N.getBestModeForData(t);if(n=N.from(e,a),n!==N.BYTE&&n.bit<a.bit)throw new Error('"'+t+'" cannot be encoded with mode '+N.toString(n)+`.
 Suggested mode is: `+N.toString(a));switch(n===N.KANJI&&!Pe.isKanjiModeEnabled()&&(n=N.BYTE),n){case N.NUMERIC:return new Vn(t);case N.ALPHANUMERIC:return new zn(t);case N.KANJI:return new Hn(t);case N.BYTE:return new Gn(t)}}te.fromArray=function(e){return e.reduce(function(n,a){return typeof a=="string"?n.push(kn(a,null)):a.data&&n.push(kn(a.data,a.mode)),n},[])};te.fromString=function(e,n){let a=Kn(e,Pe.isKanjiModeEnabled()),r=Bd(a),o=Fd(r,n),i=Ud.find_path(o.map,"start","end"),s=[];for(let c=1;c<i.length-1;c++)s.push(o.table[i[c]].node);return te.fromArray(Pd(s))};te.rawSplit=function(e){return te.fromArray(Kn(e,Pe.isKanjiModeEnabled()))}});var Yn=S(qn=>{var Fe=$(),dt=be(),$d=on(),kd=cn(),Vd=ln(),zd=un(),Rt=Rn(),Et=Qe(),Gd=Nn(),Be=In(),Hd=bn(),Kd=V(),_t=Wn();function Wd(t,e){let n=t.size,a=zd.getPositions(e);for(let r=0;r<a.length;r++){let o=a[r][0],i=a[r][1];for(let s=-1;s<=7;s++)if(!(o+s<=-1||n<=o+s))for(let c=-1;c<=7;c++)i+c<=-1||n<=i+c||(s>=0&&s<=6&&(c===0||c===6)||c>=0&&c<=6&&(s===0||s===6)||s>=2&&s<=4&&c>=2&&c<=4?t.set(o+s,i+c,!0,!0):t.set(o+s,i+c,!1,!0))}}function qd(t){let e=t.size;for(let n=8;n<e-8;n++){let a=n%2===0;t.set(n,6,a,!0),t.set(6,n,a,!0)}}function Yd(t,e){let n=Vd.getPositions(e);for(let a=0;a<n.length;a++){let r=n[a][0],o=n[a][1];for(let i=-2;i<=2;i++)for(let s=-2;s<=2;s++)i===-2||i===2||s===-2||s===2||i===0&&s===0?t.set(r+i,o+s,!0,!0):t.set(r+i,o+s,!1,!0)}}function Xd(t,e){let n=t.size,a=Be.getEncodedBits(e),r,o,i;for(let s=0;s<18;s++)r=Math.floor(s/3),o=s%3+n-8-3,i=(a>>s&1)===1,t.set(r,o,i,!0),t.set(o,r,i,!0)}function ut(t,e,n){let a=t.size,r=Hd.getEncodedBits(e,n),o,i;for(o=0;o<15;o++)i=(r>>o&1)===1,o<6?t.set(o,8,i,!0):o<8?t.set(o+1,8,i,!0):t.set(a-15+o,8,i,!0),o<8?t.set(8,a-o-1,i,!0):o<9?t.set(8,15-o-1+1,i,!0):t.set(8,15-o-1,i,!0);t.set(a-8,8,1,!0)}function jd(t,e){let n=t.size,a=-1,r=n-1,o=7,i=0;for(let s=n-1;s>0;s-=2)for(s===6&&s--;;){for(let c=0;c<2;c++)if(!t.isReserved(r,s-c)){let l=!1;i<e.length&&(l=(e[i]>>>o&1)===1),t.set(r,s-c,l),o--,o===-1&&(i++,o=7)}if(r+=a,r<0||n<=r){r-=a,a=-a;break}}}function Jd(t,e,n){let a=new $d;n.forEach(function(c){a.put(c.mode.bit,4),a.put(c.getLength(),Kd.getCharCountIndicator(c.mode,t)),c.write(a)});let r=Fe.getSymbolTotalCodewords(t),o=Et.getTotalCodewordsCount(t,e),i=(r-o)*8;for(a.getLengthInBits()+4<=i&&a.put(0,4);a.getLengthInBits()%8!==0;)a.putBit(0);let s=(i-a.getLengthInBits())/8;for(let c=0;c<s;c++)a.put(c%2?17:236,8);return Zd(a,t,e)}function Zd(t,e,n){let a=Fe.getSymbolTotalCodewords(e),r=Et.getTotalCodewordsCount(e,n),o=a-r,i=Et.getBlocksCount(e,n),s=a%i,c=i-s,l=Math.floor(a/i),d=Math.floor(o/i),R=d+1,_=l-d,O=new Gd(_),f=0,A=new Array(i),g=new Array(i),L=0,I=new Uint8Array(t.buffer);for(let h=0;h<i;h++){let b=h<c?d:R;A[h]=I.slice(f,f+b),g[h]=O.encode(A[h]),f+=b,L=Math.max(L,b)}let w=new Uint8Array(a),C=0,u,E;for(u=0;u<L;u++)for(E=0;E<i;E++)u<A[E].length&&(w[C++]=A[E][u]);for(u=0;u<_;u++)for(E=0;E<i;E++)w[C++]=g[E][u];return w}function Qd(t,e,n,a){let r;if(Array.isArray(t))r=_t.fromArray(t);else if(typeof t=="string"){let l=e;if(!l){let d=_t.rawSplit(t);l=Be.getBestVersionForData(d,n)}r=_t.fromString(t,l||40)}else throw new Error("Invalid data");let o=Be.getBestVersionForData(r,n);if(!o)throw new Error("The amount of data is too big to be stored in a QR Code");if(!e)e=o;else if(e<o)throw new Error(`
The chosen QR Code version cannot contain this amount of data.
Minimum version required to store current data is: `+o+`.
`);let i=Jd(e,n,r),s=Fe.getSymbolSize(e),c=new kd(s);return Wd(c,e),qd(c),Yd(c,e),ut(c,n,0),e>=7&&Xd(c,e),jd(c,i),isNaN(a)&&(a=Rt.getBestMask(c,ut.bind(null,c,n))),Rt.applyMask(a,c),ut(c,n,a),{modules:c,version:e,errorCorrectionLevel:n,maskPattern:a,segments:r}}qn.create=function(e,n){if(typeof e>"u"||e==="")throw new Error("No input text");let a=dt.M,r,o;return typeof n<"u"&&(a=dt.from(n.errorCorrectionLevel,dt.M),r=Be.from(n.version),o=Rt.from(n.maskPattern),n.toSJISFunc&&Fe.setToSJISFunction(n.toSJISFunc)),Qd(e,r,a,o)}});var ht=S(Y=>{function Xn(t){if(typeof t=="number"&&(t=t.toString()),typeof t!="string")throw new Error("Color should be defined as hex string");let e=t.slice().replace("#","").split("");if(e.length<3||e.length===5||e.length>8)throw new Error("Invalid hex color: "+t);(e.length===3||e.length===4)&&(e=Array.prototype.concat.apply([],e.map(function(a){return[a,a]}))),e.length===6&&e.push("F","F");let n=parseInt(e.join(""),16);return{r:n>>24&255,g:n>>16&255,b:n>>8&255,a:n&255,hex:"#"+e.slice(0,6).join("")}}Y.getOptions=function(e){e||(e={}),e.color||(e.color={});let n=typeof e.margin>"u"||e.margin===null||e.margin<0?4:e.margin,a=e.width&&e.width>=21?e.width:void 0,r=e.scale||4;return{width:a,scale:a?4:r,margin:n,color:{dark:Xn(e.color.dark||"#000000ff"),light:Xn(e.color.light||"#ffffffff")},type:e.type,rendererOpts:e.rendererOpts||{}}};Y.getScale=function(e,n){return n.width&&n.width>=e+n.margin*2?n.width/(e+n.margin*2):n.scale};Y.getImageWidth=function(e,n){let a=Y.getScale(e,n);return Math.floor((e+n.margin*2)*a)};Y.qrToImageData=function(e,n,a){let r=n.modules.size,o=n.modules.data,i=Y.getScale(r,a),s=Math.floor((r+a.margin*2)*i),c=a.margin*i,l=[a.color.light,a.color.dark];for(let d=0;d<s;d++)for(let R=0;R<s;R++){let _=(d*s+R)*4,O=a.color.light;if(d>=c&&R>=c&&d<s-c&&R<s-c){let f=Math.floor((d-c)/i),A=Math.floor((R-c)/i);O=l[o[f*r+A]?1:0]}e[_++]=O.r,e[_++]=O.g,e[_++]=O.b,e[_]=O.a}}});var jn=S($e=>{var At=ht();function e_(t,e,n){t.clearRect(0,0,e.width,e.height),e.style||(e.style={}),e.height=n,e.width=n,e.style.height=n+"px",e.style.width=n+"px"}function t_(){try{return document.createElement("canvas")}catch{throw new Error("You need to specify a canvas element")}}$e.render=function(e,n,a){let r=a,o=n;typeof r>"u"&&(!n||!n.getContext)&&(r=n,n=void 0),n||(o=t_()),r=At.getOptions(r);let i=At.getImageWidth(e.modules.size,r),s=o.getContext("2d"),c=s.createImageData(i,i);return At.qrToImageData(c.data,e,r),e_(s,o,i),s.putImageData(c,0,0),o};$e.renderToDataURL=function(e,n,a){let r=a;typeof r>"u"&&(!n||!n.getContext)&&(r=n,n=void 0),r||(r={});let o=$e.render(e,n,r),i=r.type||"image/png",s=r.rendererOpts||{};return o.toDataURL(i,s.quality)}});var Qn=S(Zn=>{var n_=ht();function Jn(t,e){let n=t.a/255,a=e+'="'+t.hex+'"';return n<1?a+" "+e+'-opacity="'+n.toFixed(2).slice(1)+'"':a}function Ot(t,e,n){let a=t+e;return typeof n<"u"&&(a+=" "+n),a}function a_(t,e,n){let a="",r=0,o=!1,i=0;for(let s=0;s<t.length;s++){let c=Math.floor(s%e),l=Math.floor(s/e);!c&&!o&&(o=!0),t[s]?(i++,s>0&&c>0&&t[s-1]||(a+=o?Ot("M",c+n,.5+l+n):Ot("m",r,0),r=0,o=!1),c+1<e&&t[s+1]||(a+=Ot("h",i),i=0)):r++}return a}Zn.render=function(e,n,a){let r=n_.getOptions(n),o=e.modules.size,i=e.modules.data,s=o+r.margin*2,c=r.color.light.a?"<path "+Jn(r.color.light,"fill")+' d="M0 0h'+s+"v"+s+'H0z"/>':"",l="<path "+Jn(r.color.dark,"stroke")+' d="'+a_(i,o,r.margin)+'"/>',d='viewBox="0 0 '+s+" "+s+'"',_='<svg xmlns="http://www.w3.org/2000/svg" '+(r.width?'width="'+r.width+'" height="'+r.width+'" ':"")+d+' shape-rendering="crispEdges">'+c+l+`</svg>
`;return typeof a=="function"&&a(null,_),_}});var ta=S(ue=>{var r_=nn(),Nt=Yn(),ea=jn(),o_=Qn();function St(t,e,n,a,r){let o=[].slice.call(arguments,1),i=o.length,s=typeof o[i-1]=="function";if(!s&&!r_())throw new Error("Callback required as last argument");if(s){if(i<2)throw new Error("Too few arguments provided");i===2?(r=n,n=e,e=a=void 0):i===3&&(e.getContext&&typeof r>"u"?(r=a,a=void 0):(r=a,a=n,n=e,e=void 0))}else{if(i<1)throw new Error("Too few arguments provided");return i===1?(n=e,e=a=void 0):i===2&&!e.getContext&&(a=n,n=e,e=void 0),new Promise(function(c,l){try{let d=Nt.create(n,a);c(t(d,e,a))}catch(d){l(d)}})}try{let c=Nt.create(n,a);r(null,t(c,e,a))}catch(c){r(c)}}ue.create=Nt.create;ue.toCanvas=St.bind(null,ea.render);ue.toDataURL=St.bind(null,ea.renderToDataURL);ue.toString=St.bind(null,function(t,e,n){return o_.render(t,n)})});var k_={};Sa(k_,{LocalSolanaMobileWalletAdapterWallet:()=>da,RemoteSolanaMobileWalletAdapterWallet:()=>_a,SolanaMobileWalletAdapterRemoteWalletName:()=>sa,SolanaMobileWalletAdapterWalletName:()=>ia,createDefaultAuthorizationCache:()=>F_,createDefaultChainSelector:()=>$_,createDefaultWalletNotFoundHandler:()=>B_,defaultErrorModalWalletNotFoundHandler:()=>ua,registerMwa:()=>v_});var B="solana:signAndSendTransaction";var Se="solana:signIn";var me="solana:signMessage";var F="solana:signTransaction";var fa=1,ga=2,Ta=3,Ia=4,Ca=5,wa=6,La=7,ba=8,ya=9,va=10,Da=11,xa=12,Ma=-32700,Ua=-32603,Pa=-32602,Ba=-32601,Fa=-32600,$a=-32021,ka=-32020,Va=-32019,za=-32018,Ga=-32017,Ha=-32016,Ka=-32015,Wa=-32014,qa=-32013,Ya=-32012,Xa=-32011,ja=-32010,Ja=-32009,Za=-32008,Qa=-32007,er=-32006,tr=-32005,nr=-32004,ar=-32003,rr=-32002,or=-32001,ir=28e5,sr=2800001,cr=2800002,lr=2800003,dr=2800004,_r=2800005,ur=2800006,Rr=2800007,Er=2800008,hr=2800009,Ar=2800010,Or=2800011,Nr=323e4,Sr=32300001,mr=3230002,pr=3230003,fr=3230004,gr=361e4,Tr=3610001,Ir=3610002,Cr=3610003,wr=3610004,Lr=3610005,br=3610006,yr=3610007,vr=3611e3,Dr=3704e3,xr=3704001,Mr=3704002,Ur=3704003,Pr=3704004,Br=3704005,Fr=3704006,$r=3712e3,kr=4128e3,Vr=4128001,zr=4128002,Gr=4615e3,Hr=4615001,Kr=4615002,Wr=4615003,qr=4615004,Yr=4615005,Xr=4615006,jr=4615007,Jr=4615008,Zr=4615009,Qr=4615010,eo=4615011,to=4615012,no=4615013,ao=4615014,ro=4615015,oo=4615016,io=4615017,so=4615018,co=4615019,lo=4615020,_o=4615021,uo=4615022,Ro=4615023,Eo=4615024,ho=4615025,Ao=4615026,Oo=4615027,No=4615028,So=4615029,mo=4615030,po=4615031,fo=4615032,go=4615033,To=4615034,Io=4615035,Co=4615036,wo=4615037,Lo=4615038,bo=4615039,yo=4615040,vo=4615041,Do=4615042,xo=4615043,Mo=4615044,Uo=4615045,Po=4615046,Bo=4615047,Fo=4615048,$o=4615049,ko=4615050,Vo=4615051,zo=4615052,Go=4615053,Ho=4615054,Ko=5508e3,Wo=5508001,qo=5508002,Yo=5508003,Xo=5508004,jo=5508005,Jo=5508006,Zo=5508007,Qo=5508008,ei=5508009,ti=5508010,ni=5508011,ai=5508012,ri=5607e3,oi=5607001,ii=5607002,si=5607003,ci=5607004,li=5607005,di=5607006,_i=5607007,ui=5607008,Ri=5607009,Ei=5607010,hi=5607011,Ai=5607012,Oi=5607013,Ni=5607014,Si=5607015,mi=5607016,pi=5607017,fi=5607018,gi=5607019,Ti=5663e3,Ii=5663001,Ci=5663002,wi=5663003,Li=5663004,bi=5663005,yi=5663006,vi=5663007,Di=5663008,xi=5663009,Mi=5663010,Ui=5663011,Pi=5663012,Bi=5663013,Fi=5663014,$i=5663015,ki=5663016,Vi=5663017,zi=5663018,Gi=5663019,Hi=5663020,Ki=5663021,Wi=5663022,qi=5663023,Yi=5663024,Xi=5663025,ji=5663026,Ji=5663027,Zi=5663028,Qi=5663029,es=5663030,ts=5663031,ns=5663032,as=5663033,rs=5663034,os=5663035,is=5663036,ss=5663037,cs=5663038,ls=5664e3,ds=5664001,_s=705e4,us=7050001,Rs=7050002,Es=7050003,hs=7050004,As=7050005,Os=7050006,Ns=7050007,Ss=7050008,ms=7050009,ps=7050010,fs=7050011,gs=7050012,Ts=7050013,Is=7050014,Cs=7050015,ws=7050016,Ls=7050017,bs=7050018,ys=7050019,vs=7050020,Ds=7050021,xs=7050022,Ms=7050023,Us=7050024,Ps=7050025,Bs=7050026,Fs=7050027,$s=7050028,ks=7050029,Vs=7050030,zs=7050031,Gs=7050032,Hs=7050033,Ks=7050034,Ws=7050035,qs=7050036,Ys=7618e3,Xs=7618001,js=7618002,Js=7618003,Zs=7618004,Qs=7618005,ec=7618006,tc=7618007,nc=7618008,ac=7618009,rc=7618010,oc=7618011,ic=8078e3,sc=8078001,cc=8078002,lc=8078003,dc=8078004,_c=8078005,uc=8078006,Rc=8078007,Ec=8078008,hc=8078009,Ac=8078010,Oc=8078011,pe=8078012,Nc=8078013,Sc=8078014,mc=8078015,pc=8078016,fc=8078017,gc=8078018,Tc=8078019,Ic=8078020,Cc=8078021,wc=8078022,Lc=8078023,bc=8078024,yc=8078025,vc=809e4,Dc=8090001,xc=8090002,Mc=8090003,Uc=8090004,Pc=8090005,Bc=8090006,Fc=8090007,$c=8090008,kc=8090009,Vc=8090010,zc=8090011,Gc=8090012,Hc=81e5,Kc=8100001,Wc=8100002,qc=8100003,Yc=819e4,Xc=8190001,jc=8190002,Jc=8190003,Zc=8190004,Qc=8195e3,el=8195001,tl=85e5,nl=8500001,al=8500002,rl=8500003,ol=8500004,il=8500005,sl=8500006,cl=89e5,ll=8900001,dl=8900002,_l=8900003,ul=9e6,Rl=9000001,El=9000002,hl=99e5,Al=9900001,Ol=9900002,Nl=9900003,Sl=9900004,ml=9900005,pl=9900006;function Tt(t){return Array.isArray(t)?"%5B"+t.map(Tt).join("%2C%20")+"%5D":typeof t=="bigint"?`${t}n`:encodeURIComponent(String(t!=null&&Object.getPrototypeOf(t)===null?{...t}:t))}function fl([t,e]){return`${t}=${Tt(e)}`}function gl(t){let e=Object.entries(t).map(fl).join("&");return btoa(e)}var W_={[Nr]:"Account not found at address: $address",[fr]:"Not all accounts were decoded. Encoded accounts found at addresses: $addresses.",[pr]:"Expected decoded account at address: $address",[mr]:"Failed to decode account data at address: $address",[Sr]:"Accounts not found at addresses: $addresses",[hr]:"Unable to find a viable program address bump seed.",[cr]:"$putativeAddress is not a base58-encoded address.",[ir]:"Expected base58 encoded address to decode to a byte array of length 32. Actual length: $actualLength.",[lr]:"The `CryptoKey` must be an `Ed25519` public key.",[Or]:"$putativeOffCurveAddress is not a base58-encoded off-curve address.",[Er]:"Invalid seeds; point must fall off the Ed25519 curve.",[dr]:"Expected given program derived address to have the following format: [Address, ProgramDerivedAddressBump].",[ur]:"A maximum of $maxSeeds seeds, including the bump seed, may be supplied when creating an address. Received: $actual.",[Rr]:"The seed at index $index with length $actual exceeds the maximum length of $maxSeedLength bytes.",[_r]:"Expected program derived address bump to be in the range [0, 255], got: $bump.",[Ar]:"Program address cannot end with PDA marker.",[sr]:"Expected base58-encoded address string of length in the range [32, 44]. Actual length: $actualLength.",[Ia]:"Expected base58-encoded blockhash string of length in the range [32, 44]. Actual length: $actualLength.",[fa]:"The network has progressed past the last block for which this transaction could have been committed.",[ic]:"Codec [$codecDescription] cannot decode empty byte arrays.",[wc]:"Enum codec cannot use lexical values [$stringValues] as discriminators. Either remove all lexical values or set `useValuesAsDiscriminators` to `false`.",[Ic]:"Sentinel [$hexSentinel] must not be present in encoded bytes [$hexEncodedBytes].",[_c]:"Encoder and decoder must have the same fixed size, got [$encoderFixedSize] and [$decoderFixedSize].",[uc]:"Encoder and decoder must have the same max size, got [$encoderMaxSize] and [$decoderMaxSize].",[dc]:"Encoder and decoder must either both be fixed-size or variable-size.",[Ec]:"Enum discriminator out of range. Expected a number in [$formattedValidDiscriminators], got $discriminator.",[cc]:"Expected a fixed-size codec, got a variable-size one.",[Nc]:"Codec [$codecDescription] expected a positive byte length, got $bytesLength.",[lc]:"Expected a variable-size codec, got a fixed-size one.",[Tc]:"Codec [$codecDescription] expected zero-value [$hexZeroValue] to have the same size as the provided fixed-size item [$expectedSize bytes].",[sc]:"Codec [$codecDescription] expected $expected bytes, got $bytesLength.",[gc]:"Expected byte array constant [$hexConstant] to be present in data [$hexData] at offset [$offset].",[hc]:"Invalid discriminated union variant. Expected one of [$variants], got $value.",[Ac]:"Invalid enum variant. Expected one of [$stringValues] or a number in [$formattedNumericalValues], got $variant.",[mc]:"Invalid literal union variant. Expected one of [$variants], got $value.",[Rc]:"Expected [$codecDescription] to have $expected items, got $actual.",[pe]:"Invalid value $value for base $base with alphabet $alphabet.",[pc]:"Literal union discriminator out of range. Expected a number between $minRange and $maxRange, got $discriminator.",[Oc]:"Codec [$codecDescription] expected number to be in the range [$min, $max], got $value.",[Sc]:"Codec [$codecDescription] expected offset to be in the range [0, $bytesLength], got $offset.",[Cc]:"Expected sentinel [$hexSentinel] to be present in decoded bytes [$hexDecodedBytes].",[fc]:"Union variant out of range. Expected an index between $minRange and $maxRange, got $variant.",[Lc]:"This decoder expected a byte array of exactly $expectedLength bytes, but $numExcessBytes unexpected excess bytes remained after decoding. Are you sure that you have chosen the correct decoder for this data?",[bc]:"Invalid pattern match value. The provided value does not match any of the specified patterns.",[yc]:"Invalid pattern match bytes. The provided byte array does not match any of the specified patterns.",[vr]:"No random values implementation could be found.",[Da]:"Failed to send transaction$causeMessage",[xa]:"Failed to send transactions$causeMessages",[Fc]:"Fixed-point operation `$operation` of kind `$kind` overflowed. Expected a raw bigint in [$min, $max], got $result.",[kc]:"Fixed-point division by zero for value of kind `$kind` ($signedness, $totalBits bits).",[Mc]:"`fractionalBits` ($fractionalBits) must not exceed `totalBits` ($totalBits).",[xc]:"Invalid `decimals`. Expected a non-negative integer, got $decimals.",[Dc]:"Invalid `fractionalBits`. Expected a non-negative integer, got $fractionalBits.",[Pc]:"Invalid string `$input` for fixed-point value of kind `$kind`.",[vc]:"Invalid `totalBits`. Expected a positive integer, got $totalBits.",[Bc]:"Invalid ratio $numerator/$denominator for fixed-point value of kind `$kind`. Denominator must be non-zero.",[zc]:"Fixed-point value of kind `$kind` has a malformed `raw` field. Expected a bigint, got `$raw`.",[$c]:"Fixed-point `$operation` operation expected $expectedKind ($expectedSignedness, $expectedTotalBits bits, $expectedScale $expectedScaleLabel); got $actualKind ($actualSignedness, $actualTotalBits bits, $actualScale $actualScaleLabel).",[Vc]:"Fixed-point operation `$operation` of kind `$kind` cannot be performed exactly; pass a rounding mode other than `strict` to allow a rounded result.",[Gc]:"Fixed-point codec of kind `$kind` requires `totalBits` to be a multiple of 8; got $totalBits.",[Uc]:"Fixed-point value of kind `$kind` is out of range for $signedness $totalBits-bit storage. Expected a raw bigint in [$min, $max], got $raw.",[$r]:"Filesystem operation `$operation` is not supported in this environment.",[Zr]:"Instruction requires an uninitialized account",[Ro]:"Instruction tries to borrow reference for an account which is already borrowed",[Eo]:"Instruction left account with an outstanding borrowed reference",[_o]:"Program other than the account's owner changed the size of the account data",[Yr]:"Account data too small for instruction",[uo]:"Instruction expected an executable account",[Po]:"An account does not have enough lamports to be rent-exempt",[Fo]:"Program arithmetic overflowed",[Uo]:"Failed to serialize or deserialize account data",[Ho]:"Builtin programs must consume compute units",[fo]:"Cross-program invocation call depth too deep",[Lo]:"Computational budget exceeded",[Ao]:"Custom program error: #$code",[io]:"Instruction contains duplicate accounts",[ho]:"Instruction modifications of multiply-passed account differ",[mo]:"Executable accounts must be rent exempt",[No]:"Instruction changed executable accounts data",[So]:"Instruction changed the balance of an executable account",[so]:"Instruction changed executable bit of an account",[ao]:"Instruction modified data of an account it does not own",[no]:"Instruction spent from the balance of an account it does not own",[Hr]:"Generic instruction error",[ko]:"Provided owner is not allowed",[xo]:"Account is immutable",[Mo]:"Incorrect authority provided",[jr]:"Incorrect program id for instruction",[Xr]:"Insufficient funds for instruction",[qr]:"Invalid account data for instruction",[Bo]:"Invalid account owner",[Kr]:"Invalid program argument",[Oo]:"Program returned invalid error code",[Wr]:"Invalid instruction data",[wo]:"Failed to reallocate account data",[Co]:"Provided seeds do not result in a valid address",[Vo]:"Accounts data allocations exceeded the maximum allowed per transaction",[zo]:"Max accounts exceeded",[Go]:"Max instruction trace length exceeded",[Io]:"Length of the seed is too long for address generation",[go]:"An account required by the instruction is missing",[Jr]:"Missing required signature for instruction",[to]:"Instruction illegally modified the program id of an account",[lo]:"Insufficient account keys for instruction",[bo]:"Cross-program invocation with unauthorized signer or writable account",[yo]:"Failed to create program execution environment",[Do]:"Program failed to compile",[vo]:"Program failed to complete",[oo]:"Instruction modified data of a read-only account",[ro]:"Instruction changed the balance of a read-only account",[To]:"Cross-program invocation reentrancy not allowed for this instruction",[co]:"Instruction modified rent epoch of an account",[eo]:"Sum of account balances before and after instruction do not match",[Qr]:"Instruction requires an initialized account",[Gr]:"The instruction failed with the error: $errorName",[po]:"Unsupported program id",[$o]:"Unsupported sysvar",[ml]:"Invalid instruction plan kind: $kind.",[js]:"The provided instruction plan is empty.",[Qs]:"No failed transaction plan result was found in the provided transaction plan result.",[Zs]:"This transaction plan executor does not support non-divisible sequential plans. To support them, you may create your own executor such that multi-transaction atomicity is preserved \u2014 e.g. by targetting RPCs that support transaction bundles.",[Js]:"The provided transaction plan failed to execute. See the `transactionPlanResult` attribute for more details. Note that the `cause` property is deprecated, and a future version will not set it.",[oc]:"The configured maximum of $maxInstructions instructions per transaction is invalid. It must be a positive integer no greater than the transaction format limit of $transactionInstructionLimit instructions per transaction. Provide a `maxInstructionsPerTransaction` (on the transaction planner) or `maxInstructions` (on the message packer) value between 1 and $transactionInstructionLimit.",[rc]:"Planning this transaction message would require $numInstructions instructions, which exceeds the configured maximum of $maxInstructions instructions per transaction. This limit is configurable, and intended to leave headroom for inner instructions which are included in the maximum instruction limit for transactions. Increase `maxInstructionsPerTransaction` on the transaction planner (or `maxInstructions` on the message packer) to allow more instructions per transaction.",[Ys]:"The provided message has insufficient capacity to accommodate the next instruction(s) in this plan. Expected at least $numBytesRequired free byte(s), got $numFreeBytes byte(s).",[pl]:"Invalid transaction plan kind: $kind.",[Xs]:"No more instructions to pack; the message packer has completed the instruction plan.",[ec]:"Unexpected instruction plan. Expected $expectedKind plan, got $actualKind plan.",[tc]:"Unexpected transaction plan. Expected $expectedKind plan, got $actualKind plan.",[nc]:"Unexpected transaction plan result. Expected $expectedKind plan, got $actualKind plan.",[ac]:"Expected a successful transaction plan result. I.e. there is at least one failed or cancelled transaction in the plan.",[kr]:"The instruction does not have any accounts.",[Vr]:"The instruction does not have any data.",[zr]:"Expected instruction to have progress address $expectedProgramAddress, got $actualProgramAddress.",[Ca]:"Expected base58 encoded blockhash to decode to a byte array of length 32. Actual length: $actualLength.",[ga]:"The nonce `$expectedNonceValue` is no longer valid. It has advanced to `$actualNonceValue`",[Ol]:"Invariant violation: Found no abortable iterable cache entry for key `$cacheKey`. It should be impossible to hit this error; please file an issue at https://sola.na/web3invariant",[Sl]:"Invariant violation: This data publisher does not publish to the channel named `$channelName`. Supported channels include $supportedChannelNames.",[Al]:"Invariant violation: WebSocket message iterator state is corrupt; iterated without first resolving existing message promise. It should be impossible to hit this error; please file an issue at https://sola.na/web3invariant",[hl]:"Invariant violation: WebSocket message iterator is missing state storage. It should be impossible to hit this error; please file an issue at https://sola.na/web3invariant",[Nl]:"Invariant violation: Switch statement non-exhaustive. Received unexpected value `$unexpectedValue`. It should be impossible to hit this error; please file an issue at https://sola.na/web3invariant",[Ua]:"JSON-RPC error: Internal JSON-RPC error ($__serverMessage)",[Pa]:"JSON-RPC error: Invalid method parameter(s) ($__serverMessage)",[Fa]:"JSON-RPC error: The JSON sent is not a valid `Request` object ($__serverMessage)",[Ba]:"JSON-RPC error: The method does not exist / is not available ($__serverMessage)",[Ma]:"JSON-RPC error: An error occurred on the server while parsing the JSON text ($__serverMessage)",[Ya]:"$__serverMessage",[or]:"$__serverMessage",[nr]:"$__serverMessage",[Wa]:"$__serverMessage",[Ga]:"Epoch rewards period still active at slot $slot",[ka]:"$__serverMessage",[ja]:"$__serverMessage",[Ja]:"$__serverMessage",[Va]:"Failed to query long-term storage; please try again",[Ha]:"Minimum context slot has not been reached",[tr]:"Node is unhealthy; behind by $numSlotsBehind slots",[$a]:"No slot history",[Za]:"No snapshot",[rr]:"Transaction simulation failed",[za]:"Rewards cannot be found because slot $slot is not the epoch boundary. This may be due to gap in the queried node's local ledger or long-term storage",[Qa]:"$__serverMessage",[Xa]:"Transaction history is not available from this node",[er]:"$__serverMessage",[qa]:"Transaction signature length mismatch",[ar]:"Transaction signature verification failure",[Ka]:"$__serverMessage",[Br]:"The grind regex `/$source/` contains the character `$character`, which is not in the base58 alphabet and can never match a Solana address.",[Dr]:"Key pair bytes must be of length 64, got $byteLength.",[xr]:"Expected private key bytes with length 32. Actual length: $actualLength.",[Mr]:"Expected base58-encoded signature to decode to a byte array of length 64. Actual length: $actualLength.",[Pr]:"The provided private key does not match the provided public key.",[Ur]:"Expected base58-encoded signature string of length in the range [64, 88]. Actual length: $actualLength.",[Fr]:"Writing a key pair to disk is not supported in this environment.",[wa]:"Lamports value must be in the range [0, 2e64-1]",[La]:"`$value` cannot be parsed as a `BigInt`",[va]:"$message",[ba]:"`$value` cannot be parsed as a `Number`",[Ta]:"No nonce account could be found at address `$nonceAccountAddress`",[si]:"Expected base58 encoded application domain to decode to a byte array of length 32. Actual length: $actualLength.",[Oi]:"Attempted to sign an offchain message with an address that is not a signer for it",[ii]:"Expected base58-encoded application domain string of length in the range [32, 44]. Actual length: $actualLength.",[fi]:"The content of the offchain message does not match the content that was expected. Expected content with a byte-length of $expectedBytes; got content with a byte-length of $actualBytes. The signer may have signed different data than was requested; do not trust its signature.",[Ai]:"The signer addresses in this offchain message envelope do not match the list of required signers in the message preamble. These unexpected signers were present in the envelope: `[$unexpectedSigners]`. These required signers were missing from the envelope `[$missingSigners]`.",[ri]:"The message body provided has a byte-length of $actualBytes. The maximum allowable byte-length is $maxBytes",[_i]:"Expected message format $expectedMessageFormat, got $actualMessageFormat",[ui]:"The message length specified in the message preamble is $specifiedLength bytes. The actual length of the message is $actualLength bytes.",[Ri]:"Offchain message content must be non-empty",[li]:"Offchain message must specify the address of at least one required signer",[Ei]:"Offchain message envelope must reserve space for at least one signature",[ci]:"The offchain message preamble specifies $numRequiredSignatures required signature(s), got $signaturesLength.",[gi]:"The offchain message lists different required signatories than was expected. Expected [$expectedAddresses]. Got [$actualAddresses]. The signer may have signed different data than was requested; do not trust its signature.",[Si]:"The signatories of this offchain message must be listed in lexicographical order",[mi]:"An address must be listed no more than once among the signatories of an offchain message",[hi]:"Offchain message is missing signatures for addresses: $addresses.",[pi]:"Offchain message signature verification failed. Signature mismatch for required signatories [$signatoriesWithInvalidSignatures]. Missing signatures for signatories [$signatoriesWithMissingSignatures]",[oi]:"The message body provided contains characters whose codes fall outside the allowed range. In order to ensure clear-signing compatiblity with hardware wallets, the message may only contain line feeds and characters in the range [\\x20-\\x7e].",[Ni]:"Expected offchain message version $expectedVersion. Got $actualVersion.",[di]:"This version of Kit does not support decoding offchain messages with version $unsupportedVersion. The current max supported version is 0.",[sl]:"The provided account could not be identified as an account from the $programName program.",[al]:"The provided instruction could not be identified as an instruction from the $programName program.",[tl]:"The provided instruction is missing some accounts. Expected at least $expectedAccountMetas account(s), got $actualAccountMetas.",[ol]:"Expected resolved instruction input '$inputName' to be non-null.",[rl]:"Expected resolved instruction input '$inputName' to be of type `$expectedType`.",[il]:"Unrecognized account type '$accountType' for the $programName program.",[nl]:"Unrecognized instruction type '$instructionType' for the $programName program.",[Yc]:"The notification name must end in 'Notifications' and the API must supply a subscription plan creator function for the notification '$notificationName'.",[jc]:"WebSocket was closed before payload could be added to the send buffer",[Jc]:"WebSocket connection closed",[Zc]:"WebSocket failed to connect",[Xc]:"Failed to obtain a subscription id from the server",[qc]:"Could not find an API plan for RPC method: `$method`",[Hc]:"The $argumentLabel argument to the `$methodName` RPC method$optionalPathLabel was `$value`. This number is unsafe for use with the Solana JSON-RPC because it exceeds `Number.MAX_SAFE_INTEGER`.",[Wc]:"HTTP error ($statusCode): $message",[Kc]:"HTTP header(s) forbidden: $headers. Learn more at https://developer.mozilla.org/en-US/docs/Glossary/Forbidden_header_name.",[Ko]:"Multiple distinct signers were identified for address `$address`. Please ensure that you are using the same signer instance for each address.",[Wo]:"The provided value does not implement the `KeyPairSigner` interface",[Yo]:"The provided value does not implement the `MessageModifyingSigner` interface",[Xo]:"The provided value does not implement the `MessagePartialSigner` interface",[qo]:"The provided value does not implement any of the `MessageSigner` interfaces",[Jo]:"The provided value does not implement the `TransactionModifyingSigner` interface",[Zo]:"The provided value does not implement the `TransactionPartialSigner` interface",[Qo]:"The provided value does not implement the `TransactionSendingSigner` interface",[jo]:"The provided value does not implement any of the `TransactionSigner` interfaces",[ei]:"More than one `TransactionSendingSigner` was identified.",[ti]:"No `TransactionSendingSigner` was identified. Please provide a valid `TransactionWithSingleSendingSigner` transaction.",[ai]:"The wallet account $address cannot be used to create a transaction signer because it does not implement either the `solana:signTransaction` or `solana:signAndSendTransaction` feature. At least one of these features is required. The account supports the following features: $supportedFeatures.",[ni]:"Wallet account signers do not support signing multiple messages/transactions in a single operation",[Qc]:"This `ReactiveStreamStore` does not support retry. Use `createReactiveStoreFromDataPublisherFactory` to construct a retryable store.",[el]:"The stream store closed in an error state but did not report an error.",[yr]:"Cannot export a non-extractable key.",[Tr]:"No digest implementation could be found.",[gr]:"Cryptographic operations are only allowed in secure browser contexts. Read more here: https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts.",[Ir]:`This runtime does not support the generation of Ed25519 key pairs.

Install @solana/webcrypto-ed25519-polyfill and call its \`install\` function before generating keys in environments that do not support Ed25519.

For a list of runtimes that currently support Ed25519 operations, visit https://github.com/WICG/webcrypto-secure-curves/issues/20.`,[Cr]:"No key export implementation could be found.",[wr]:"No key generation implementation could be found.",[Lr]:"No signing implementation could be found.",[br]:"No signature verification implementation could be found.",[ya]:"Timestamp value must be in the range [-(2n ** 63n), (2n ** 63n) - 1]. `$value` given",[ws]:"Transaction processing left an account with an outstanding borrowed reference",[us]:"Account in use",[Rs]:"Account loaded twice",[Es]:"Attempt to debit an account but found no record of a prior credit.",[Ms]:"Transaction loads an address table account that doesn't exist",[Ns]:"This transaction has already been processed",[Ss]:"Blockhash not found",[ms]:"Loader call chain is too deep",[Cs]:"Transactions are currently disabled due to cluster maintenance",[Vs]:"Transaction contains a duplicate instruction ($index) that is not allowed",[As]:"Insufficient funds for fee",[zs]:"Transaction results in an account ($accountIndex) with insufficient funds for rent",[Os]:"This account may not be used to pay transaction fees",[fs]:"Transaction contains an invalid account reference",[Ps]:"Transaction loads an address table account with invalid data",[Bs]:"Transaction address table lookup uses an invalid index",[Us]:"Transaction loads an address table account with an invalid owner",[Hs]:"LoadedAccountsDataSizeLimit set for transaction must be greater than 0.",[Ts]:"This program may not be used for executing instructions",[Fs]:"Transaction leaves an account with a lower balance than rent-exempt minimum",[ys]:"Transaction loads a writable account that cannot be written",[Gs]:"Transaction exceeded max loaded accounts data size cap",[ps]:"Transaction requires a fee but has no signature present",[hs]:"Attempt to load a program that does not exist",[Ws]:"Execution of the program referenced by account at index $accountIndex is temporarily restricted.",[Ks]:"ResanitizationNeeded",[Is]:"Transaction failed to sanitize accounts offsets correctly",[gs]:"Transaction did not pass signature verification",[xs]:"Transaction locked too many accounts",[qs]:"Sum of account balances before and after transaction do not match",[_s]:"The transaction failed with the error `$errorName`",[bs]:"Transaction version is unsupported",[Ds]:"Transaction would exceed account data limit within the block",[ks]:"Transaction would exceed total account data limit",[vs]:"Transaction would exceed max account limit within the block",[Ls]:"Transaction would exceed max Block Cost Limit",[$s]:"Transaction would exceed max Vote Cost Limit",[$i]:"Attempted to sign a transaction with an address that is not a signer for it",[Mi]:"Transaction is missing an address at index: $index.",[ki]:"Transaction has no expected signers therefore it cannot be encoded",[Hi]:"Transaction size $transactionSize exceeds limit of $transactionSizeLimit bytes",[Ci]:"Transaction does not have a blockhash lifetime",[wi]:"Transaction is not a durable nonce transaction",[bi]:"Contents of these address lookup tables unknown: $lookupTableAddresses",[yi]:"Lookup of address at index $highestRequestedIndex failed for lookup table `$lookupTableAddress`. Highest known index is $highestKnownIndex. The lookup table may have been extended since its contents were retrieved",[Di]:"No fee payer set in CompiledTransaction",[vi]:"Could not find program address at index $index",[zi]:"Failed to estimate the compute unit consumption for this transaction message. This is likely because simulating the transaction failed. Inspect the `cause` property of this error to learn more",[is]:"Failed to estimate the loaded accounts data size for this transaction message. The RPC did not return a `loadedAccountsDataSize` value from simulation. This value is required for version 1 transactions",[Gi]:"Transaction failed when it was simulated in order to estimate the compute unit consumption. The compute unit estimate provided is for a transaction that failed when simulated and may not be representative of the compute units this transaction would consume if successful. Inspect the `cause` property of this error to learn more",[ss]:"Transaction failed when it was simulated in order to estimate its resource limits. The resource limit estimates provided are for a transaction that failed when simulated and may not be representative of the resources this transaction would consume if successful. Inspect the `cause` property of this error to learn more",[Ui]:"Transaction is missing a fee payer.",[Pi]:"Could not determine this transaction's signature. Make sure that the transaction has been signed by its fee payer.",[Fi]:"Transaction first instruction is not advance nonce account instruction.",[Bi]:"Transaction with no instructions cannot be durable nonce transaction.",[Ti]:"This transaction includes an address (`$programAddress`) which is both invoked and set as the fee payer. Program addresses may not pay fees",[Ii]:"This transaction includes an address (`$programAddress`) which is both invoked and marked writable. Program addresses may not be writable",[Vi]:"The transaction message expected the transaction to have $numRequiredSignatures signatures, got $signaturesLength.",[xi]:"Transaction is missing signatures for addresses: $addresses.",[Li]:"Transaction version must be in the range [0, 127]. `$actualVersion` given",[Ki]:"This version of Kit does not support decoding transactions with version $unsupportedVersion. The current max supported version is 1.",[Wi]:"The transaction has a durable nonce lifetime (with nonce `$nonce`), but the nonce account address is in a lookup table. The lifetime constraint cannot be constructed without fetching the lookup tables for the transaction.",[Zi]:"Invalid transaction config mask: $mask. Bits 0 and 1 must match (both set or both unset)",[qi]:"Transaction message bytes are malformed: $messageBytes",[Yi]:"Transaction message bytes are empty, so the transaction cannot be encoded",[Xi]:"Transaction bytes are empty, so no transaction can be decoded",[ji]:"Transaction version 0 must be encoded with signatures first. This transaction was encoded with first byte $firstByte, which is expected to be a signature count for v0 transactions.",[Ji]:"The provided transaction bytes expect that there should be $numExpectedSignatures signatures, but the bytes are not long enough to contain a transaction message with this many signatures. The provided bytes are $transactionBytesLength bytes long.",[Qi]:"The transaction has a durable nonce lifetime, but the nonce account index is invalid. Expected a nonce account index less than $numberOfStaticAccounts, got $nonceAccountIndex.",[es]:"The transaction config value for $configName has the incorrect kind. Expected $expectedKind, got $actualKind.",[ts]:"The transaction does not have the same number of instruction headers and instruction payloads. Got $numInstructionHeaders instruction headers, and $numInstructionPayloads instruction payloads.",[ns]:"Transaction has $actualCount unique signer addresses but the maximum allowed is $maxAllowed",[as]:"Transaction has $actualCount unique account addresses but the maximum allowed is $maxAllowed",[rs]:"Transaction has $actualCount instructions but the maximum allowed is $maxAllowed",[os]:"The instruction at index $instructionIndex has $actualCount account references but the maximum allowed is $maxAllowed",[cs]:"Could not find an account address at index $index while decompiling an instruction",[ls]:"`getTransaction` responses fetched with `encoding: 'jsonParsed'` cannot be decoded. Re-fetch the transaction with `encoding: 'base64'`, `'base58'`, or `'json'`",[ds]:"Could not recognize the shape of this `getTransaction` response. Expected a response fetched with `encoding: 'base64'`, `'base58'`, or `'json'`",[Rl]:"`$hookName` requires the following capabilities to be installed on the client: [$capabilities]. $providerHint",[ul]:"`$hookName` was called outside of a `ClientProvider`. Mount a `<ClientProvider client={client}>` in the ancestor tree.",[El]:"The subscription closed in an error state but did not report an error.",[cl]:"Cannot $operation: no wallet connected",[ll]:"No signing wallet connected (status: $status)",[dl]:"Connected wallet does not support signing",[_l]:'Account $address is not available in wallet "$walletName"'};function Tl(t,e={}){{let n=`Solana error #${t}; Decode this error by running \`npx @solana/errors decode -- ${t}`;return Object.keys(e).length&&(n+=` '${gl(e)}'`),`${n}\``}}var Ve=class extends Error{cause=this.cause;context;constructor(...[t,e]){let n,a;e&&Object.entries(Object.getOwnPropertyDescriptors(e)).forEach(([o,i])=>{o==="cause"?a={cause:i.value}:(n===void 0&&(n={__code:t}),Object.defineProperty(n,o,i))});let r=Tl(t,n);super(r,a),this.context=Object.freeze(n===void 0?{__code:t}:n),this.name="SolanaError"}};function Il(t,e){return"fixedSize"in e?e.fixedSize:e.getSizeFromValue(t)}function ze(t){return Object.freeze({...t,encode:e=>{let n=new Uint8Array(Il(e,t));return t.write(e,n,0),n}})}function fe(t){return Object.freeze({...t,decode:(e,n=0)=>t.read(e,n)[0]})}var Cl=t=>fe({read(e,n){let a=n===0||n<=-e.byteLength?e:e.slice(n);if(a.length===0)return["",e.length];let r=a.findIndex(c=>c!==0);r=r===-1?a.length:r;let o=t[0].repeat(r);if(r===a.length)return[o,e.length];let i=a.slice(r).reduce((c,l)=>c*256n+BigInt(l),0n),s=wl(i,t);return[o+s,e.length]}});function wl(t,e){let n=BigInt(e.length),a=[];for(;t>0n;)a.unshift(e[Number(t%n)]),t/=n;return a.join("")}var Ll="123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";var wt=()=>Cl(Ll);var It="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",Lt=()=>ze({getSizeFromValue:t=>{try{return atob(t).length}catch{throw new Ve(pe,{alphabet:It,base:64,value:t})}},write(t,e,n){try{let a=atob(t).split("").map(r=>r.charCodeAt(0));return e.set(a,n),a.length+n}catch{throw new Ve(pe,{alphabet:It,base:64,value:t})}}}),Ge=()=>fe({read(t,e=0){let n=t.slice(e);return[btoa(String.fromCharCode(...n)),t.length]}});var bl=t=>t.replace(/\u0000/g,"");var yl=globalThis.TextDecoder,Ct=globalThis.TextEncoder,He=()=>{let t;return ze({getSizeFromValue:e=>(t||=new Ct).encode(e).length,write:(e,n,a)=>{let r=(t||=new Ct).encode(e);return n.set(r,a),a+r.length}})},bt=()=>{let t;return fe({read(e,n){let a=(t||=new yl).decode(e.slice(n));return[bl(a),e.length]}})};function yt(t){return Ge().decode(He().encode(t))}function H(t,e){let n=Ge().decode(t);return e?n.replace(/\+/g,"-").replace(/\//g,"_").replace(/=+$/,""):n}function ge(t){return Lt().encode(t)}function vl(t){return wt().decode(t)}function vt(t){return vl(ge(t))}function Ke(t){return H(new Uint8Array(t))}function Dt(t){return bt().decode(t)}function xt(t){return He().encode(t)}var We="solana:mainnet";var Dl="(?<domain>[^\\n]+?) wants you to sign in with your Solana account:\\n",xl="(?<address>[^\\n]+)(?:\\n|$)",Ml="(?:\\n(?<statement>[\\S\\s]*?)(?:\\n|$))??",Ul="(?:\\nURI: (?<uri>[^\\n]+))?",Pl="(?:\\nVersion: (?<version>[^\\n]+))?",Bl="(?:\\nChain ID: (?<chainId>[^\\n]+))?",Fl="(?:\\nNonce: (?<nonce>[^\\n]+))?",$l="(?:\\nIssued At: (?<issuedAt>[^\\n]+))?",kl="(?:\\nExpiration Time: (?<expirationTime>[^\\n]+))?",Vl="(?:\\nNot Before: (?<notBefore>[^\\n]+))?",zl="(?:\\nRequest ID: (?<requestId>[^\\n]+))?",Gl="(?:\\nResources:(?<resources>(?:\\n- [^\\n]+)*))?",Hl=`${Ul}${Pl}${Bl}${Fl}${$l}${kl}${Vl}${zl}${Gl}`,ru=new RegExp(`^${Dl}${xl}${Ml}${Hl}\\n*$`);function Mt(t){let e=`${t.domain} wants you to sign in with your Solana account:
`;e+=`${t.address}`,t.statement&&(e+=`

${t.statement}`);let n=[];if(t.uri&&n.push(`URI: ${t.uri}`),t.version&&n.push(`Version: ${t.version}`),t.chainId&&n.push(`Chain ID: ${t.chainId}`),t.nonce&&n.push(`Nonce: ${t.nonce}`),t.issuedAt&&n.push(`Issued At: ${t.issuedAt}`),t.expirationTime&&n.push(`Expiration Time: ${t.expirationTime}`),t.notBefore&&n.push(`Not Before: ${t.notBefore}`),t.requestId&&n.push(`Request ID: ${t.requestId}`),t.resources){n.push("Resources:");for(let a of t.resources)n.push(`- ${a}`)}return n.length&&(e+=`

${n.join(`
`)}`),e}var p={ERROR_ASSOCIATION_PORT_OUT_OF_RANGE:"ERROR_ASSOCIATION_PORT_OUT_OF_RANGE",ERROR_REFLECTOR_ID_OUT_OF_RANGE:"ERROR_REFLECTOR_ID_OUT_OF_RANGE",ERROR_FORBIDDEN_WALLET_BASE_URL:"ERROR_FORBIDDEN_WALLET_BASE_URL",ERROR_SECURE_CONTEXT_REQUIRED:"ERROR_SECURE_CONTEXT_REQUIRED",ERROR_SESSION_CLOSED:"ERROR_SESSION_CLOSED",ERROR_SESSION_TIMEOUT:"ERROR_SESSION_TIMEOUT",ERROR_WALLET_NOT_FOUND:"ERROR_WALLET_NOT_FOUND",ERROR_INVALID_PROTOCOL_VERSION:"ERROR_INVALID_PROTOCOL_VERSION",ERROR_BROWSER_NOT_SUPPORTED:"ERROR_BROWSER_NOT_SUPPORTED",ERROR_LOOPBACK_ACCESS_BLOCKED:"ERROR_LOOPBACK_ACCESS_BLOCKED",ERROR_ASSOCIATION_CANCELLED:"ERROR_ASSOCIATION_CANCELLED",ERROR_ILLEGAL_TRANSPORT_STATE:"ERROR_ILLEGAL_TRANSPORT_STATE"},m=class extends Error{data;code;constructor(...t){let[e,n,a]=t;super(n),this.code=e,this.data=a,this.name="SolanaMobileWalletAdapterError"}};var qe=class extends Error{data;code;jsonRpcMessageId;constructor(...t){let[e,n,a,r]=t;super(a),this.code=n,this.data=r,this.jsonRpcMessageId=e,this.name="SolanaMobileWalletAdapterProtocolError"}};async function Te(t,e){let n=await crypto.subtle.exportKey("raw",t),a=await crypto.subtle.sign({hash:"SHA-256",name:"ECDSA"},e,n),r=new Uint8Array(n.byteLength+a.byteLength);return r.set(new Uint8Array(n),0),r.set(new Uint8Array(a),n.byteLength),r}function Kl(t){return Mt(t)}function Wl(t){return yt(Kl(t)).replace(/\+/g,"-").replace(/\//g,"_").replace(/=+$/,"")}var ql="solana:signTransactions",Ut="solana:cloneAuthorization";function Ft(t,e){return new Proxy({},{get(n,a){return a==="then"?null:(n[a]==null&&(n[a]=async function(r){let{method:o,params:i}=Yl(a,r,t),s=await e(o,i);return o==="authorize"&&i.sign_in_payload&&!s.sign_in_result&&(s.sign_in_result=await jl(i.sign_in_payload,s,e)),Xl(a,s,t)}),n[a])},defineProperty(){return!1},deleteProperty(){return!1}})}function Yl(t,e,n){let a=e,r=t.toString().replace(/[A-Z]/g,o=>`_${o.toLowerCase()}`).toLowerCase();switch(t){case"authorize":{let o=a,{chain:i}=o;if(n==="legacy"){switch(i){case"solana:testnet":i="testnet";break;case"solana:devnet":i="devnet";break;case"solana:mainnet":i="mainnet-beta";break;default:i=o.cluster}o.cluster=i,a=o}else{switch(i){case"testnet":case"devnet":i=`solana:${i}`;break;case"mainnet-beta":i="solana:mainnet"}o.chain=i,a=o}}case"reauthorize":{let{auth_token:o,identity:i}=a;o&&(n==="legacy"?(r="reauthorize",a={auth_token:o,identity:i}):r="authorize");break}}return{method:r,params:a}}function Xl(t,e,n){switch(t){case"getCapabilities":{let a=e;switch(n){case"legacy":{let r=[ql];return a.supports_clone_authorization===!0&&r.push(Ut),{...a,features:r}}case"v1":return{...a,supports_sign_and_send_transactions:!0,supports_clone_authorization:a.features.includes(Ut)}}}}return e}async function jl(t,e,n){let a=t.domain??window.location.host,r=e.accounts[0].address,o=Wl({...t,domain:a,address:vt(r)}),i=await n("sign_messages",{addresses:[r],payloads:[o]}),s=ge(i.signed_payloads[0]),c=H(s.slice(0,s.length-64)),l=H(s.slice(s.length-64));return{address:r,signed_message:c.length==0?o:c,signature:l}}function Jl(t){if(t>=4294967296)throw new Error("Outbound sequence number overflow. The maximum sequence number is 32-bytes.");let e=new ArrayBuffer(4);return new DataView(e).setUint32(0,t,!1),new Uint8Array(e)}var Zl=12;async function Ql(t,e,n){let a=Jl(e),r=new Uint8Array(Zl);crypto.getRandomValues(r);let o=await crypto.subtle.encrypt(kt(a,r),n,xt(t)),i=new Uint8Array(a.byteLength+r.byteLength+o.byteLength);return i.set(new Uint8Array(a),0),i.set(new Uint8Array(r),a.byteLength),i.set(new Uint8Array(o),a.byteLength+r.byteLength),i}async function $t(t,e){let n=t.slice(0,4),a=t.slice(4,16),r=t.slice(16),o=await crypto.subtle.decrypt(kt(n,a),e,r);return Dt(new Uint8Array(o))}function kt(t,e){return{additionalData:t,iv:e,name:"AES-GCM",tagLength:128}}async function Vt(){return await crypto.subtle.generateKey({name:"ECDSA",namedCurve:"P-256"},!1,["sign"])}async function Ie(){return await crypto.subtle.generateKey({name:"ECDH",namedCurve:"P-256"},!1,["deriveKey","deriveBits"])}function ed(){return zt(49152+Math.floor(Math.random()*16384))}function zt(t){if(t<49152||t>65535)throw new m(p.ERROR_ASSOCIATION_PORT_OUT_OF_RANGE,`Association port number must be between 49152 and 65535. ${t} given.`,{port:t});return t}function Gt(t){return t.replace(/[/+=]/g,e=>({"/":"_","+":"-","=":"."})[e])}var td="solana-wallet";function Pt(t){return t.replace(/(^\/+|\/+$)/g,"").split("/")}function Ht(t,e){let n=null;if(e){try{n=new URL(e)}catch{}if(n?.protocol!=="https:")throw new m(p.ERROR_FORBIDDEN_WALLET_BASE_URL,"Base URLs supplied by wallets must be valid `https` URLs")}n||=new URL(`${td}:/`);let a=t.startsWith("/")?t:[...Pt(n.pathname),...Pt(t)].join("/");return new URL(a,n)}async function nd(t,e,n,a=["v1"]){let r=zt(e),o=await crypto.subtle.exportKey("raw",t),i=Ke(o),s=Ht("v1/associate/local",n);return s.searchParams.set("association",Gt(i)),s.searchParams.set("port",`${r}`),a.forEach(c=>{s.searchParams.set("v",c)}),s}async function ad(t,e,n,a,r=["v1"]){let o=await crypto.subtle.exportKey("raw",t),i=Ke(o),s=Ht("v1/associate/remote",a);return s.searchParams.set("association",Gt(i)),s.searchParams.set("reflector",`${e}`),s.searchParams.set("id",`${H(n,!0)}`),r.forEach(c=>{s.searchParams.set("v",c)}),s}async function Kt(t,e){let n=JSON.stringify(t),a=t.id;return Ql(n,a,e)}async function Wt(t,e){let n=await $t(t,e),a=JSON.parse(n);if(Object.hasOwnProperty.call(a,"error"))throw new qe(a.id,a.error.code,a.error.message);return a}async function qt(t,e,n){let[a,r]=await Promise.all([crypto.subtle.exportKey("raw",e),crypto.subtle.importKey("raw",t.slice(0,65),{name:"ECDH",namedCurve:"P-256"},!1,[])]),o=await crypto.subtle.deriveBits({name:"ECDH",public:r},n,256),i=await crypto.subtle.importKey("raw",o,"HKDF",!1,["deriveKey"]);return await crypto.subtle.deriveKey({name:"HKDF",hash:"SHA-256",salt:new Uint8Array(a),info:new Uint8Array},i,{name:"AES-GCM",length:128},!1,["encrypt","decrypt"])}async function Yt(t,e){let n=await $t(t,e),a=JSON.parse(n),r="legacy";if(Object.hasOwnProperty.call(a,"v"))switch(a.v){case 1:case"1":case"v1":r="v1";break;case"legacy":r="legacy";break;default:throw new m(p.ERROR_INVALID_PROTOCOL_VERSION,`Unknown/unsupported protocol version: ${a.v}`)}return{protocol_version:r}}var Ce={Firefox:0,Other:1};function rd(){return navigator.userAgent.indexOf("Firefox/")!==-1?Ce.Firefox:Ce.Other}function od(){return new Promise((t,e)=>{function n(){clearTimeout(r),window.removeEventListener("blur",a)}function a(){n(),t()}window.addEventListener("blur",a);let r=setTimeout(()=>{n(),e()},3e3)})}var X=null;function id(t){(X==null||!X.isConnected)&&(X=document.createElement("iframe"),X.style.display="none",document.body.appendChild(X)),X.contentWindow.location.href=t.toString()}async function sd(t){if(t.protocol==="https:")window.location.assign(t);else try{switch(rd()){case Ce.Firefox:id(t);break;case Ce.Other:{let e=od();window.location.assign(t),await e;break}}}catch{throw new m(p.ERROR_WALLET_NOT_FOUND,"Found no installed wallet that supports the mobile wallet protocol.")}}async function cd(t,e){let n=ed();return await sd(await nd(t,n,e)),n}var we={retryDelayScheduleMs:[150,150,200,500,500,750,750,1e3],timeoutMs:3e4},Xt="com.solana.mobilewalletadapter.v1",Bt="com.solana.mobilewalletadapter.v1.base64";function jt(){if(typeof window>"u"||window.isSecureContext!==!0)throw new m(p.ERROR_SECURE_CONTEXT_REQUIRED,"The mobile wallet adapter protocol must be used in a secure context (`https`).")}function Jt(t){let e;try{e=new URL(t)}catch{throw new m(p.ERROR_FORBIDDEN_WALLET_BASE_URL,"Invalid base URL supplied by wallet")}if(e.protocol!=="https:")throw new m(p.ERROR_FORBIDDEN_WALLET_BASE_URL,"Base URLs supplied by wallets must be valid `https` URLs")}function Le(t){return new DataView(t).getUint32(0,!1)}function ld(t){let e=new Uint8Array(t),n=t.byteLength,a=10,r=0,o=0,i;do{if(o>=n||o>a)throw new RangeError("Failed to decode varint");i=e[o++],r|=(i&127)<<7*o}while(i>=128);return{value:r,offset:o}}function dd(t){let{value:e,offset:n}=ld(t);return new Uint8Array(t.slice(n,n+e))}async function Zt(t){jt();let e=await Vt(),n=`ws://localhost:${await cd(e.publicKey,t?.baseUri)}/solana-wallet`,a,r=(()=>{let R=[...we.retryDelayScheduleMs];return()=>R.length>1?R.shift():R[0]})(),o=1,i=0,s={__type:"disconnected"},c,l=!1,d;return{close:()=>{c.close(),d()},wallet:new Promise((R,_)=>{let O={},f=async()=>{if(s.__type!=="connecting"){console.warn(`Expected adapter state to be \`connecting\` at the moment the websocket opens. Got \`${s.__type}\`.`);return}c.removeEventListener("open",f);let{associationKeypair:u}=s,E=await Ie();c.send(await Te(E.publicKey,u.privateKey)),s={__type:"hello_req_sent",associationPublicKey:u.publicKey,ecdhPrivateKey:E.privateKey}},A=u=>{u.wasClean?s={__type:"disconnected"}:_(new m(p.ERROR_SESSION_CLOSED,`The wallet session dropped unexpectedly (${u.code}: ${u.reason}).`,{closeEvent:u})),I()},g=async u=>{I(),Date.now()-a>=we.timeoutMs?_(new m(p.ERROR_SESSION_TIMEOUT,`Failed to connect to the wallet websocket at ${n}.`)):(await new Promise(E=>{let h=r();w=window.setTimeout(E,h)}),C())},L=async u=>{let E=await u.data.arrayBuffer();switch(s.__type){case"connecting":{if(E.byteLength!==0){_(new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encountered unexpected message while connecting"));return}let h=await Ie();c.send(await Te(h.publicKey,e.privateKey)),s={__type:"hello_req_sent",associationPublicKey:e.publicKey,ecdhPrivateKey:h.privateKey};break}case"connected":try{let h=Le(E.slice(0,4));if(h!==i+1)throw new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encrypted message has invalid sequence number");i=h;let b=await Wt(E,s.sharedSecret),v=O[b.id];delete O[b.id],v.resolve(b.result)}catch(h){if(h instanceof qe){let b=O[h.jsonRpcMessageId];delete O[h.jsonRpcMessageId],b.reject(h)}else throw h}break;case"hello_req_sent":{if(E.byteLength===0){let M=await Ie();c.send(await Te(M.publicKey,e.privateKey)),s={__type:"hello_req_sent",associationPublicKey:e.publicKey,ecdhPrivateKey:M.privateKey};break}let h=await qt(E,s.associationPublicKey,s.ecdhPrivateKey),b=E.slice(65),v=b.byteLength!==0?await(async()=>{let M=Le(b.slice(0,4));return M!==i+1?(_(new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encrypted message has invalid sequence number")),c.close(),{protocol_version:"v1"}):(i=M,Yt(b,h))})():{protocol_version:"legacy"};s={__type:"connected",sharedSecret:h,sessionProperties:v};let ne=Ft(v.protocol_version,async(M,Ee)=>{let he=o++;return c.send(await Kt({id:he,jsonrpc:"2.0",method:M,params:Ee??{}},h)),new Promise((Ae,ae)=>{O[he]={resolve(re){switch(M){case"authorize":case"reauthorize":{let{wallet_uri_base:Oe}=re;if(Oe!=null)try{Jt(Oe)}catch(Ra){ae(Ra);return}break}}Ae(re)},reject:ae}})});l=!0;try{R(ne)}catch(M){_(M)}break}}};d=()=>{c.removeEventListener("message",L),I(),l||_(new m(p.ERROR_SESSION_CLOSED,"The wallet session was closed before connection.",{closeEvent:new CloseEvent("socket was closed before connection")}))};let I,w,C=()=>{I&&I(),s={__type:"connecting",associationKeypair:e},a===void 0&&(a=Date.now()),c=new WebSocket(n,[Xt]),c.addEventListener("open",f),c.addEventListener("close",A),c.addEventListener("error",g),c.addEventListener("message",L),I=()=>{window.clearTimeout(w),c.removeEventListener("open",f),c.removeEventListener("close",A),c.removeEventListener("error",g),c.removeEventListener("message",L)}};C()})}}async function Qt(t){jt();let e=await Vt(),n=`wss://${t?.remoteHostAuthority}/reflect`,a,r=(()=>{let A=[...we.retryDelayScheduleMs];return()=>A.length>1?A.shift():A[0]})(),o=1,i=0,s,c={__type:"disconnected"},l,d,R=async A=>{if(s=="base64"){let g=await A.data;return ge(g).buffer}else return await A.data.arrayBuffer()},_=await new Promise((A,g)=>{let L=async()=>{if(c.__type!=="connecting"){console.warn(`Expected adapter state to be \`connecting\` at the moment the websocket opens. Got \`${c.__type}\`.`);return}l.protocol.includes(Bt)?s="base64":s="binary",l.removeEventListener("open",L)},I=h=>{h.wasClean?c={__type:"disconnected"}:g(new m(p.ERROR_SESSION_CLOSED,`The wallet session dropped unexpectedly (${h.code}: ${h.reason}).`,{closeEvent:h})),d()},w=async h=>{d(),Date.now()-a>=we.timeoutMs?g(new m(p.ERROR_SESSION_TIMEOUT,`Failed to connect to the wallet websocket at ${n}.`)):(await new Promise(b=>{let v=r();u=window.setTimeout(b,v)}),E())},C=async h=>{let b=await R(h);if(c.__type==="connecting"){if(b.byteLength==0){g(new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encountered unexpected message while connecting")),l.close();return}let v=dd(b);c={__type:"reflector_id_received",reflectorId:v};let ne=await ad(e.publicKey,t.remoteHostAuthority,v,t?.baseUri);l.removeEventListener("message",C),A(ne)}},u,E=()=>{d&&d(),c={__type:"connecting",associationKeypair:e},a===void 0&&(a=Date.now()),l=new WebSocket(n,[Xt,Bt]),l.addEventListener("open",L),l.addEventListener("close",I),l.addEventListener("error",w),l.addEventListener("message",C),d=()=>{window.clearTimeout(u),l.removeEventListener("open",L),l.removeEventListener("close",I),l.removeEventListener("error",w),l.removeEventListener("message",C)}};E()}),O=!1,f;return{associationUrl:_,close:()=>{l.close(),f()},wallet:new Promise((A,g)=>{let L={},I=async w=>{let C=await R(w);switch(c.__type){case"reflector_id_received":{if(C.byteLength!==0){g(new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encountered unexpected message while awaiting reflection")),l.close();return}let u=await Ie(),E=await Te(u.publicKey,e.privateKey);s=="base64"?l.send(H(E)):l.send(E),c={__type:"hello_req_sent",associationPublicKey:e.publicKey,ecdhPrivateKey:u.privateKey};break}case"connected":try{let u=Le(C.slice(0,4));if(u!==i+1)throw new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encrypted message has invalid sequence number");i=u;let E=await Wt(C,c.sharedSecret),h=L[E.id];delete L[E.id],h.resolve(E.result)}catch(u){if(u instanceof qe){let E=L[u.jsonRpcMessageId];delete L[u.jsonRpcMessageId],E.reject(u)}else throw u}break;case"hello_req_sent":{let u=await qt(C,c.associationPublicKey,c.ecdhPrivateKey),E=C.slice(65),h=E.byteLength!==0?await(async()=>{let v=Le(E.slice(0,4));return v!==i+1?(g(new m(p.ERROR_ILLEGAL_TRANSPORT_STATE,"Encrypted message has invalid sequence number")),l.close(),{protocol_version:"v1"}):(i=v,Yt(E,u))})():{protocol_version:"legacy"};c={__type:"connected",sharedSecret:u,sessionProperties:h};let b=Ft(h.protocol_version,async(v,ne)=>{let M=o++,Ee=await Kt({id:M,jsonrpc:"2.0",method:v,params:ne??{}},u);return s=="base64"?l.send(H(Ee)):l.send(Ee),new Promise((he,Ae)=>{L[M]={resolve(ae){switch(v){case"authorize":case"reauthorize":{let{wallet_uri_base:re}=ae;if(re!=null)try{Jt(re)}catch(Oe){Ae(Oe);return}break}}he(ae)},reject:Ae}})});O=!0;try{A(b)}catch(v){g(v)}break}}};l.addEventListener("message",I),f=()=>{l.removeEventListener("message",I),d(),O||g(new m(p.ERROR_SESSION_CLOSED,"The wallet session was closed before connection.",{closeEvent:new CloseEvent("socket was closed before connection")}))}})}}var Ye="standard:connect";var Xe="standard:disconnect";var je="standard:events";function _d(t){if(t.length>=255)throw new TypeError("Alphabet too long");let e=new Uint8Array(256);for(let l=0;l<e.length;l++)e[l]=255;for(let l=0;l<t.length;l++){let d=t.charAt(l),R=d.charCodeAt(0);if(e[R]!==255)throw new TypeError(d+" is ambiguous");e[R]=l}let n=t.length,a=t.charAt(0),r=Math.log(n)/Math.log(256),o=Math.log(256)/Math.log(n);function i(l){if(l instanceof Uint8Array||(ArrayBuffer.isView(l)?l=new Uint8Array(l.buffer,l.byteOffset,l.byteLength):Array.isArray(l)&&(l=Uint8Array.from(l))),!(l instanceof Uint8Array))throw new TypeError("Expected Uint8Array");if(l.length===0)return"";let d=0,R=0,_=0,O=l.length;for(;_!==O&&l[_]===0;)_++,d++;let f=(O-_)*o+1>>>0,A=new Uint8Array(f);for(;_!==O;){let I=l[_],w=0;for(let C=f-1;(I!==0||w<R)&&C!==-1;C--,w++)I+=256*A[C]>>>0,A[C]=I%n>>>0,I=I/n>>>0;if(I!==0)throw new Error("Non-zero carry");R=w,_++}let g=f-R;for(;g!==f&&A[g]===0;)g++;let L=a.repeat(d);for(;g<f;++g)L+=t.charAt(A[g]);return L}function s(l){if(typeof l!="string")throw new TypeError("Expected String");if(l.length===0)return new Uint8Array;let d=0,R=0,_=0;for(;l[d]===a;)R++,d++;let O=(l.length-d)*r+1>>>0,f=new Uint8Array(O);for(;d<l.length;){let I=l.charCodeAt(d);if(I>255)return;let w=e[I];if(w===255)return;let C=0;for(let u=O-1;(w!==0||C<_)&&u!==-1;u--,C++)w+=n*f[u]>>>0,f[u]=w%256>>>0,w=w/256>>>0;if(w!==0)throw new Error("Non-zero carry");_=C,d++}let A=O-_;for(;A!==O&&f[A]===0;)A++;let g=new Uint8Array(R+(O-A)),L=R;for(;A!==O;)g[L++]=f[A++];return g}function c(l){let d=s(l);if(d)return d;throw new Error("Non-base"+n+" character")}return{encode:i,decodeUnsafe:s,decode:c}}var en=_d;var ud="123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",j=en(ud);var na=ma(ta(),1);var i_=function(t,e,n,a){if(n==="a"&&!a)throw new TypeError("Private accessor was defined without a getter");if(typeof e=="function"?t!==e||!a:!e.has(t))throw new TypeError("Cannot read private member from an object whose class did not declare it");return n==="m"?a:n==="a"?a.call(t):a?a.value:e.get(t)},s_=function(t,e,n,a,r){if(a==="m")throw new TypeError("Private method is not writable");if(a==="a"&&!r)throw new TypeError("Private accessor was defined without a setter");if(typeof e=="function"?t!==e||!r:!e.has(t))throw new TypeError("Cannot write private member to an object whose class did not declare it");return a==="a"?r.call(t,n):r?r.value=n:e.set(t,n),n},ke;function pt(t){let e=({register:n})=>n(t);try{window.dispatchEvent(new mt(e))}catch(n){console.error(`wallet-standard:register-wallet event could not be dispatched
`,n)}try{window.addEventListener("wallet-standard:app-ready",({detail:n})=>e(n))}catch(n){console.error(`wallet-standard:app-ready event listener could not be added
`,n)}}var mt=class extends Event{get detail(){return i_(this,ke,"f")}get type(){return"wallet-standard:register-wallet"}constructor(e){super("wallet-standard:register-wallet",{bubbles:!1,cancelable:!1,composed:!1}),ke.set(this,void 0),s_(this,ke,e,"f")}preventDefault(){throw new Error("preventDefault cannot be called")}stopImmediatePropagation(){throw new Error("stopImmediatePropagation cannot be called")}stopPropagation(){throw new Error("stopPropagation cannot be called")}};ke=new WeakMap;function G(t){return window.btoa(String.fromCharCode.call(null,...t))}function D(t){return new Uint8Array(window.atob(t).split("").map(e=>e.charCodeAt(0)))}var c_=`
<div class="mobile-wallet-adapter-embedded-loading-indicator" role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <div data-modal-close style="position: absolute; width: 100%; height: 100%;"></div>
    <div class="mobile-wallet-adapter-embedded-loading-container">
        <div class="mobile-wallet-adapter-embedded-loading-animation"></div>
    </div>
</div>
`,l_=`
.mobile-wallet-adapter-embedded-loading-indicator {
    display: flex; /* Use flexbox to center content */
    justify-content: center; /* Center horizontally */
    align-items: start; /* Center vertically */
    position: fixed; /* Stay in place */
    z-index: 1; /* Sit on top */
    left: 0;
    top: 0;
    width: 100%; /* Full width */
    height: 100%; /* Full height */
    background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
    overflow-y: auto; /* enable scrolling */
}

.mobile-wallet-adapter-embedded-loading-container {
    display: flex;
    margin: auto;
}

.mobile-wallet-adapter-embedded-loading-animation {
    position: relative;
    left: -9999px;
    width: 10px;
    height: 10px;
    border-radius: 5px;
    background-color: var(--spinner-color);
    color: var(--spinner-color);
    box-shadow: 9984px 0 0 0 var(--spinner-color), 
                9999px 0 0 0 var(--spinner-color), 
                10014px 0 0 0 var(--spinner-color);
    animation: dot-typing 1.5s infinite linear;
}

@keyframes dot-typing {
    0% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
    16.667% {
        box-shadow: 9984px -10px 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
    33.333% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
    50% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px -10px 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
    66.667% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
    83.333% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px -10px 0 0 var(--spinner-color);
    }
    100% {
        box-shadow: 9984px 0 0 0 var(--spinner-color), 
                    9999px 0 0 0 var(--spinner-color), 
                    10014px 0 0 0 var(--spinner-color);
    }
}
`,d_=class{#e=null;#n={};#r=!1;dom=null;constructor(){this.init=this.init.bind(this),this.#e=document.getElementById("mobile-wallet-adapter-embedded-root-ui")}async init(){console.log("Injecting modal"),this.#d()}open=()=>{console.debug("Modal open"),this.#_(),this.#e&&(this.#e.style.display="flex")};close=(t=void 0)=>{console.debug("Modal close"),this.#s(),this.#e&&(this.#e.style.display="none"),this.#n.close?.forEach(e=>e(t))};addEventListener(t,e){return this.#n[t]?.push(e)||(this.#n[t]=[e]),()=>this.removeEventListener(t,e)}removeEventListener(t,e){this.#n[t]=this.#n[t]?.filter(n=>e!==n)}#d(){if(this.dom)return;this.#e=document.createElement("div"),this.#e.id="mobile-wallet-adapter-embedded-root-ui",this.#e.innerHTML=c_,this.#e.style.display="none";let t=document.createElement("style");t.id="mobile-wallet-adapter-embedded-modal-styles",t.textContent=l_;let e=document.createElement("div");this.dom=e.attachShadow({mode:"closed"}),e.style.setProperty("--spinner-color","#FFFFFF"),this.dom.appendChild(t),this.dom.appendChild(this.#e),document.body.appendChild(e)}#_(){!this.#e||this.#r||([...this.#e.querySelectorAll("[data-modal-close]")].forEach(t=>t?.addEventListener("click",e=>{this.close(e)})),window.addEventListener("load",this.close),document.addEventListener("keydown",this.#t),this.#r=!0)}#s(){this.#r&&(window.removeEventListener("load",this.close),document.removeEventListener("keydown",this.#t),this.#e&&([...this.#e.querySelectorAll("[data-modal-close]")].forEach(t=>t?.removeEventListener("click",this.close)),this.#r=!1))}#t=t=>{t.key==="Escape"&&this.close(t)}},__=`
<div class="mobile-wallet-adapter-embedded-modal-container" role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <div data-modal-close style="position: absolute; width: 100%; height: 100%;"></div>
	<div class="mobile-wallet-adapter-embedded-modal-card">
		<div>
			<button data-modal-close class="mobile-wallet-adapter-embedded-modal-close">
				<svg width="14" height="14">
					<path d="M 6.7125,8.3036995 1.9082,13.108199 c -0.2113,0.2112 -0.4765,0.3168 -0.7957,0.3168 -0.3192,0 -0.5844,-0.1056 -0.7958,-0.3168 C 0.1056,12.896899 0,12.631699 0,12.312499 c 0,-0.3192 0.1056,-0.5844 0.3167,-0.7958 L 5.1212,6.7124995 0.3167,1.9082 C 0.1056,1.6969 0,1.4317 0,1.1125 0,0.7933 0.1056,0.5281 0.3167,0.3167 0.5281,0.1056 0.7933,0 1.1125,0 1.4317,0 1.6969,0.1056 1.9082,0.3167 L 6.7125,5.1212 11.5167,0.3167 C 11.7281,0.1056 11.9933,0 12.3125,0 c 0.3192,0 0.5844,0.1056 0.7957,0.3167 0.2112,0.2114 0.3168,0.4766 0.3168,0.7958 0,0.3192 -0.1056,0.5844 -0.3168,0.7957 L 8.3037001,6.7124995 13.1082,11.516699 c 0.2112,0.2114 0.3168,0.4766 0.3168,0.7958 0,0.3192 -0.1056,0.5844 -0.3168,0.7957 -0.2113,0.2112 -0.4765,0.3168 -0.7957,0.3168 -0.3192,0 -0.5844,-0.1056 -0.7958,-0.3168 z" />
				</svg>
			</button>
		</div>
		<div class="mobile-wallet-adapter-embedded-modal-content"></div>
	</div>
</div>
`,u_=`
.mobile-wallet-adapter-embedded-modal-container {
    display: flex; /* Use flexbox to center content */
    justify-content: center; /* Center horizontally */
    align-items: center; /* Center vertically */
    position: fixed; /* Stay in place */
    z-index: 2147483647; /* Sit on top */
    left: 0;
    top: 0;
    width: 100%; /* Full width */
    height: 100%; /* Full height */
    background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
    overflow-y: auto; /* enable scrolling */
}

.mobile-wallet-adapter-embedded-modal-card {
    display: flex;
    flex-direction: column;
    margin: auto 20px;
    max-width: 780px;
    padding: 20px;
    border-radius: 24px;
    background: #ffffff;
    font-family: "Inter Tight", "PT Sans", Calibri, sans-serif;
    transform: translateY(-200%);
    animation: slide-in 0.5s forwards;
}

@keyframes slide-in {
    100% { transform: translateY(0%); }
}

.mobile-wallet-adapter-embedded-modal-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    cursor: pointer;
    background: #e4e9e9;
    border: none;
    border-radius: 50%;
}

.mobile-wallet-adapter-embedded-modal-close:focus-visible {
    outline-color: red;
}

.mobile-wallet-adapter-embedded-modal-close svg {
    fill: #546266;
    transition: fill 200ms ease 0s;
}

.mobile-wallet-adapter-embedded-modal-close:hover svg {
    fill: #fff;
}
`,R_=`
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter+Tight:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet">
`,Re=class{#e=null;#n={};#r=!1;dom=null;constructor(){this.init=this.init.bind(this),this.#e=document.getElementById("mobile-wallet-adapter-embedded-root-ui")}async init(){console.log("Injecting modal"),this.#d()}open=()=>{console.debug("Modal open"),this.#_(),this.#e&&(this.#e.style.display="flex")};close=(t=void 0)=>{console.debug("Modal close"),this.#s(),this.#e&&(this.#e.style.display="none"),this.#n.close?.forEach(e=>e(t))};addEventListener(t,e){return this.#n[t]?.push(e)||(this.#n[t]=[e]),()=>this.removeEventListener(t,e)}removeEventListener(t,e){this.#n[t]=this.#n[t]?.filter(n=>e!==n)}#d(){if(document.getElementById("mobile-wallet-adapter-embedded-root-ui")){this.#e||(this.#e=document.getElementById("mobile-wallet-adapter-embedded-root-ui"));return}this.#e=document.createElement("div"),this.#e.id="mobile-wallet-adapter-embedded-root-ui",this.#e.innerHTML=__,this.#e.style.display="none";let t=this.#e.querySelector(".mobile-wallet-adapter-embedded-modal-content");t&&(t.innerHTML=this.contentHtml);let e=document.createElement("style");e.id="mobile-wallet-adapter-embedded-modal-styles",e.textContent=u_+this.contentStyles;let n=document.createElement("div");n.innerHTML=R_,this.dom=n.attachShadow({mode:"closed"}),this.dom.appendChild(e),this.dom.appendChild(this.#e),document.body.appendChild(n)}#_(){!this.#e||this.#r||([...this.#e.querySelectorAll("[data-modal-close]")].forEach(t=>t?.addEventListener("click",this.close)),window.addEventListener("load",this.close),document.addEventListener("keydown",this.#t),this.#r=!0)}#s(){this.#r&&(window.removeEventListener("load",this.close),document.removeEventListener("keydown",this.#t),this.#e&&([...this.#e.querySelectorAll("[data-modal-close]")].forEach(t=>t?.removeEventListener("click",this.close)),this.#r=!1))}#t=t=>{t.key==="Escape"&&this.close(t)}},E_=class extends Re{contentStyles=A_;contentHtml=h_;async initWithQR(t){super.init(),this.populateQRCode(t)}async populateQRCode(t){let e=this.dom?.getElementById("mobile-wallet-adapter-embedded-modal-qr-code-container");if(e){let n=await na.default.toCanvas(t,{width:200,margin:0});e.firstElementChild!==null?e.replaceChild(n,e.firstElementChild):e.appendChild(n);let a=this.dom?.getElementById("mobile-wallet-adapter-embedded-modal-qr-placeholder");a&&(a.style.display="none")}else console.error("QRCode Container not found")}},h_=`
<div class="mobile-wallet-adapter-embedded-modal-qr-content">
    <div>
        <svg class="mobile-wallet-adapter-embedded-modal-icon" width="100%" height="100%">
            <circle r="52" cx="53" cy="53" fill="#99b3be" stroke="#000000" stroke-width="2"/>
            <path d="m 53,82.7305 c -3.3116,0 -6.1361,-1.169 -8.4735,-3.507 -2.338,-2.338 -3.507,-5.1625 -3.507,-8.4735 0,-3.3116 1.169,-6.1364 3.507,-8.4744 2.3374,-2.338 5.1619,-3.507 8.4735,-3.507 3.3116,0 6.1361,1.169 8.4735,3.507 2.338,2.338 3.507,5.1628 3.507,8.4744 0,3.311 -1.169,6.1355 -3.507,8.4735 -2.3374,2.338 -5.1619,3.507 -8.4735,3.507 z m 0.007,-5.25 c 1.8532,0 3.437,-0.6598 4.7512,-1.9793 1.3149,-1.3195 1.9723,-2.9058 1.9723,-4.7591 0,-1.8526 -0.6598,-3.4364 -1.9793,-4.7512 -1.3195,-1.3149 -2.9055,-1.9723 -4.7582,-1.9723 -1.8533,0 -3.437,0.6598 -4.7513,1.9793 -1.3148,1.3195 -1.9722,2.9058 -1.9722,4.7591 0,1.8527 0.6597,3.4364 1.9792,4.7512 1.3195,1.3149 2.9056,1.9723 4.7583,1.9723 z m -28,-33.5729 -3.85,-3.6347 c 4.1195,-4.025 8.8792,-7.1984 14.2791,-9.52 5.4005,-2.3223 11.2551,-3.4834 17.5639,-3.4834 6.3087,0 12.1634,1.1611 17.5639,3.4834 5.3999,2.3216 10.1596,5.495 14.2791,9.52 l -3.85,3.6347 C 77.2999,40.358 73.0684,37.5726 68.2985,35.5514 63.5292,33.5301 58.4296,32.5195 53,32.5195 c -5.4297,0 -10.5292,1.0106 -15.2985,3.0319 -4.7699,2.0212 -9.0014,4.8066 -12.6945,8.3562 z m 44.625,10.8771 c -2.2709,-2.1046 -4.7962,-3.7167 -7.5758,-4.8361 -2.7795,-1.12 -5.7983,-1.68 -9.0562,-1.68 -3.2579,0 -6.2621,0.56 -9.0125,1.68 -2.7504,1.1194 -5.2903,2.7315 -7.6195,4.8361 L 32.5189,51.15 c 2.8355,-2.6028 5.9777,-4.6086 9.4263,-6.0174 3.4481,-1.4087 7.133,-2.1131 11.0548,-2.1131 3.9217,0 7.5979,0.7044 11.0285,2.1131 3.43,1.4088 6.5631,3.4146 9.3992,6.0174 z"/>
        </svg>
        <div class="mobile-wallet-adapter-embedded-modal-title">Remote Mobile Wallet Adapter</div>
    </div>
    <div>
        <div>
            <h4 class="mobile-wallet-adapter-embedded-modal-qr-label">
                Open your wallet and scan this code
            </h4>
        </div>
        <div id="mobile-wallet-adapter-embedded-modal-qr-code-container" class="mobile-wallet-adapter-embedded-modal-qr-code-container">
            <div id="mobile-wallet-adapter-embedded-modal-qr-placeholder" class="mobile-wallet-adapter-embedded-modal-qr-placeholder"></div>
        </div>
    </div>
</div>
<div class="mobile-wallet-adapter-embedded-modal-divider"><hr></div>
<div class="mobile-wallet-adapter-embedded-modal-footer">
    <div class="mobile-wallet-adapter-embedded-modal-subtitle">
        Follow the instructions on your device. When you're finished, this screen will update.
    </div>
    <div class="mobile-wallet-adapter-embedded-modal-progress-badge">
        <div>
            <div class="spinner">
                <div class="leftWrapper">
                    <div class="left">
                        <div class="circle"></div>
                    </div>
                </div>
                <div class="rightWrapper">
                    <div class="right">
                        <div class="circle"></div>
                    </div>
                </div>
            </div>
        </div>
        <div>Waiting for scan</div>
    </div>
</div>
`,A_=`
.mobile-wallet-adapter-embedded-modal-qr-content {
    display: flex; 
    margin-top: 10px;
    padding: 10px;
}

.mobile-wallet-adapter-embedded-modal-qr-content > div:first-child {
    display: flex;
    flex-direction: column;
    flex: 2;
    margin-top: auto;
    margin-right: 30px;
}

.mobile-wallet-adapter-embedded-modal-qr-content > div:nth-child(2) {
    display: flex;
    flex-direction: column;
    flex: 1;
    margin-left: auto;
}

.mobile-wallet-adapter-embedded-modal-footer {
    display: flex;
    padding: 10px;
}

.mobile-wallet-adapter-embedded-modal-icon {}

.mobile-wallet-adapter-embedded-modal-title {
    color: #000000;
    font-size: 2.5em;
    font-weight: 600;
}

.mobile-wallet-adapter-embedded-modal-qr-label {
    text-align: right;
    color: #000000;
}

.mobile-wallet-adapter-embedded-modal-qr-code-container {
    margin-left: auto;
}

.mobile-wallet-adapter-embedded-modal-qr-placeholder {
    margin-left: auto;
    min-width: 200px;
    min-height: 200px;
    background: linear-gradient(-60deg, #F7F8F8 30%, #ECEEEE 50%, #F7F8F8 70%);
    background-size: 200%;
    animation: placeholderAnimate 2.7s linear infinite;
    border-radius: 12px;
}

.mobile-wallet-adapter-embedded-modal-divider {
    margin-top: 20px;
    padding-left: 10px;
    padding-right: 10px;
}

.mobile-wallet-adapter-embedded-modal-divider hr {
    border-top: 1px solid #D9DEDE;
}

.mobile-wallet-adapter-embedded-modal-subtitle {
    margin: auto;
    margin-right: 60px;
    padding: 20px;
    color: #6E8286;
}

.mobile-wallet-adapter-embedded-modal-progress-badge {
    display: flex;
    background: #F7F8F8;
    height: 56px;
    min-width: 200px;
    margin: auto;
    padding-left: 20px;
    padding-right: 20px;
    border-radius: 18px;
    color: #A8B6B8;
    align-items: center;
}

.mobile-wallet-adapter-embedded-modal-progress-badge > div:first-child {
    margin-left: auto;
    margin-right: 20px;
}

.mobile-wallet-adapter-embedded-modal-progress-badge > div:nth-child(2) {
    margin-right: auto;
}

/* Smaller screens */
@media all and (max-width: 600px) {
    .mobile-wallet-adapter-embedded-modal-card {
        text-align: center;
    }
    .mobile-wallet-adapter-embedded-modal-qr-content {
        flex-direction: column;
    }
    .mobile-wallet-adapter-embedded-modal-qr-content > div:first-child {
        margin: auto;
    }
    .mobile-wallet-adapter-embedded-modal-qr-content > div:nth-child(2) {
        margin: auto;
        flex: 2 auto;
    }
    .mobile-wallet-adapter-embedded-modal-footer {
        flex-direction: column;
    }
    .mobile-wallet-adapter-embedded-modal-icon {
        display: none;
    }
    .mobile-wallet-adapter-embedded-modal-title {
        font-size: 1.5em;
    }
    .mobile-wallet-adapter-embedded-modal-subtitle {
        margin-right: unset;
    }
    .mobile-wallet-adapter-embedded-modal-qr-label {
        text-align: center;
    }
    .mobile-wallet-adapter-embedded-modal-qr-code-container {
        margin: auto;
    }
    .mobile-wallet-adapter-embedded-modal-qr-placeholder {
        margin: auto;
    }
}

/* QR Placeholder */
@keyframes placeholderAnimate {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
}

/* Spinner */
@keyframes spinLeft {
    0% {
        transform: rotate(20deg);
    }
    50% {
        transform: rotate(160deg);
    }
    100% {
        transform: rotate(20deg);
    }
}
@keyframes spinRight {
    0% {
        transform: rotate(160deg);
    }
    50% {
        transform: rotate(20deg);
    }
    100% {
        transform: rotate(160deg);
    }
}
@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(2520deg);
    }
}

.spinner {
    position: relative;
    width: 1.5em;
    height: 1.5em;
    margin: auto;
    animation: spin 10s linear infinite;
}
.spinner::before {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
}
.right, .rightWrapper, .left, .leftWrapper {
    position: absolute;
    top: 0;
    overflow: hidden;
    width: .75em;
    height: 1.5em;
}
.left, .leftWrapper {
    left: 0;
}
.right {
    left: -12px;
}
.rightWrapper {
    right: 0;
}
.circle {
    border: .125em solid #A8B6B8;
    width: 1.25em; /* 1.5em - 2*0.125em border */
    height: 1.25em; /* 1.5em - 2*0.125em border */
    border-radius: 0.75em; /* 0.5*1.5em spinner size 8 */
}
.left {
    transform-origin: 100% 50%;
    animation: spinLeft 2.5s cubic-bezier(.2,0,.8,1) infinite;
}
.right {
    transform-origin: 100% 50%;
    animation: spinRight 2.5s cubic-bezier(.2,0,.8,1) infinite;
}
`,O_=class extends Re{contentStyles=S_;contentHtml=N_;initWithCallback(t){super.init(),this.#e(t)}#e(t){let e=this.dom?.getElementById("mobile-wallet-adapter-launch-action"),n=async()=>{e?.removeEventListener("click",n),this.close(),t()};e?.addEventListener("click",n)}},N_=`
<svg class="mobile-wallet-adapter-embedded-modal-launch-icon" width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M21.6 48C7.2 48 0 40.8 0 26.4V21.6C0 7.2 7.2 0 21.6 0H26.4C40.8 0 48 7.2 48 21.6V26.4C48 40.8 40.8 48 26.4 48H21.6Z" fill="#15994E"/>
    <mask id="mask0_189_522" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="8" y="8" width="32" height="32">
        <rect x="8" y="8" width="32" height="32" fill="#D9D9D9"/>
    </mask>
    <g mask="url(#mask0_189_522)">
        <mask id="mask1_189_522" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="8" y="8" width="32" height="32">
            <rect x="8" y="8" width="32" height="32" fill="#D9D9D9"/>
        </mask>
        <g mask="url(#mask1_189_522)">
            <path d="M22.1092 26.1208L19.4498 23.4615C19.1736 23.1851 18.8253 23.0468 18.4048 23.0468C17.9846 23.0468 17.6363 23.1851 17.3598 23.4615C17.0836 23.7377 16.9468 24.0861 16.9495 24.5065C16.9522 24.9267 17.0916 25.275 17.3678 25.5512L21.0405 29.2238C21.3463 29.5276 21.7031 29.6795 22.1108 29.6795C22.5184 29.6795 22.8742 29.5276 23.1782 29.2238L30.5918 21.8098C30.8683 21.5336 31.0065 21.1867 31.0065 20.7692C31.0065 20.3514 30.8683 20.0044 30.5918 19.7282C30.3156 19.4517 29.9673 19.3135 29.5468 19.3135C29.1266 19.3135 28.7784 19.4517 28.5022 19.7282L22.1092 26.1208ZM23.9998 37.6042C22.113 37.6042 20.3425 37.2473 18.6885 36.5335C17.0343 35.8197 15.5954 34.8512 14.3718 33.6278C13.1485 32.4043 12.18 30.9654 11.4662 29.3112C10.7524 27.6572 10.3955 25.8867 10.3955 23.9998C10.3955 22.113 10.7524 20.3425 11.4662 18.6885C12.18 17.0343 13.1485 15.5954 14.3718 14.3718C15.5954 13.1485 17.0343 12.18 18.6885 11.4662C20.3425 10.7524 22.113 10.3955 23.9998 10.3955C25.8867 10.3955 27.6572 10.7524 29.3112 11.4662C30.9654 12.18 32.4043 13.1485 33.6278 14.3718C34.8512 15.5954 35.8197 17.0343 36.5335 18.6885C37.2473 20.3425 37.6042 22.113 37.6042 23.9998C37.6042 25.8867 37.2473 27.6572 36.5335 29.3112C35.8197 30.9654 34.8512 32.4043 33.6278 33.6278C32.4043 34.8512 30.9654 35.8197 29.3112 36.5335C27.6572 37.2473 25.8867 37.6042 23.9998 37.6042Z" fill="white"/>
        </g>
    </g>
</svg>
<div class="mobile-wallet-adapter-embedded-modal-title">Ready to connect!</div>
<div>
    <button data-modal-action id="mobile-wallet-adapter-launch-action" class="mobile-wallet-adapter-embedded-modal-launch-action">
        Connect Wallet
    </button>
</div>
`,S_=`
.mobile-wallet-adapter-embedded-modal-close {
    display: none;
}
.mobile-wallet-adapter-embedded-modal-content {
    text-align: center;
    min-width: 300px;
}
.mobile-wallet-adapter-embedded-modal-launch-icon {
    margin-top: 24px;
}
.mobile-wallet-adapter-embedded-modal-title {
    margin: 18px 100px 30px 100px;
    color: #000000;
    font-size: 2.75em;
    font-weight: 600;
}
.mobile-wallet-adapter-embedded-modal-launch-action {
    display: block;
    width: 100%;
    height: 56px;
    font-size: 1.25em;
    background: #000000;
    color: #FFFFFF;
    border-radius: 18px;
}
/* Smaller screens */
@media all and (max-width: 600px) {
    .mobile-wallet-adapter-embedded-modal-title {
        font-size: 1.5em;
        margin-right: 12px;
        margin-left: 12px;
    }
}
`,m_=class extends Re{contentStyles=f_;get contentHtml(){let t=b_()?"Long press the app icon on your home screen to open site settings":"Tap the lock or settings icon in the address bar to open site settings";return p_.replace("{{PERMISSION_INSTRUCTION_DETAIL}}",t)}async init(){super.init(),this.#e()}#e(){let t=this.dom?.getElementById("mobile-wallet-adapter-launch-action"),e=async n=>{t?.removeEventListener("click",e),this.close(n)};t?.addEventListener("click",e)}},p_=`
<div class="mobile-wallet-adapter-embedded-modal-header">
    Local Wallet Connection
</div>
<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M21.6 48C7.2 48 0 40.8 0 26.4V21.6C0 7.2 7.2 0 21.6 0H26.4C40.8 0 48 7.2 48 21.6V26.4C48 40.8 40.8 48 26.4 48H21.6Z" fill="#ED1515"/>
    <mask id="mask0_147_1364" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="8" y="8" width="32" height="32">
        <rect x="8" y="8" width="32" height="32" fill="#D9D9D9"/>
    </mask>
    <g mask="url(#mask0_147_1364)">
        <path d="M20.1398 36.2705C19.7363 36.2705 19.3508 36.1945 18.9835 36.0425C18.6162 35.8907 18.2916 35.674 18.0098 35.3922L12.6072 29.9895C12.3254 29.7077 12.1086 29.3832 11.9568 29.0158C11.8048 28.6485 11.7288 28.2631 11.7288 27.8595V20.1395C11.7288 19.736 11.8048 19.3505 11.9568 18.9832C12.1086 18.6158 12.3254 18.2913 12.6072 18.0095L18.0098 12.6068C18.2916 12.3251 18.6162 12.1083 18.9835 11.9565C19.3508 11.8045 19.7363 11.7285 20.1398 11.7285H27.8598C28.2634 11.7285 28.6488 11.8045 29.0162 11.9565C29.3835 12.1083 29.708 12.3251 29.9898 12.6068L35.3925 18.0095C35.6743 18.2913 35.891 18.6158 36.0428 18.9832C36.1948 19.3505 36.2708 19.736 36.2708 20.1395V27.8595C36.2708 28.2631 36.1948 28.6485 36.0428 29.0158C35.891 29.3832 35.6743 29.7077 35.3925 29.9895L29.9898 35.3922C29.708 35.674 29.3835 35.8907 29.0162 36.0425C28.6488 36.1945 28.2634 36.2705 27.8598 36.2705H20.1398ZM20.1732 33.2372H27.8265L33.2375 27.8262V20.1728L27.8265 14.7618H20.1732L14.7622 20.1728V27.8262L20.1732 33.2372ZM23.9998 25.9538L26.7868 28.7408C27.0473 29.0013 27.3729 29.1302 27.7638 29.1275C28.1549 29.1248 28.4807 28.9933 28.7412 28.7328C29.0016 28.4724 29.1318 28.1466 29.1318 27.7555C29.1318 27.3646 29.0016 27.039 28.7412 26.7785L25.9542 23.9995L28.7412 21.2125C29.0016 20.9521 29.1318 20.6264 29.1318 20.2355C29.1318 19.8444 29.0016 19.5186 28.7412 19.2582C28.4807 18.9977 28.1549 18.8675 27.7638 18.8675C27.3729 18.8675 27.0473 18.9977 26.7868 19.2582L23.9998 22.0452L21.2128 19.2582C20.9524 18.9977 20.628 18.8675 20.2398 18.8675C19.8514 18.8675 19.5269 18.9977 19.2665 19.2582C19.006 19.5186 18.8758 19.8444 18.8758 20.2355C18.8758 20.6264 19.006 20.9521 19.2665 21.2125L22.0455 23.9995L19.2585 26.7865C18.998 27.047 18.8692 27.3713 18.8718 27.7595C18.8745 28.148 19.006 28.4724 19.2665 28.7328C19.5269 28.9933 19.8527 29.1235 20.2438 29.1235C20.6347 29.1235 20.9604 28.9933 21.2208 28.7328L23.9998 25.9538Z" fill="black"/>
    </g>
</svg>
<div class="mobile-wallet-adapter-embedded-modal-title">
    Your wallet connection is blocked
</div>
<div id="mobile-wallet-adapter-local-launch-message" class="mobile-wallet-adapter-embedded-modal-subtitle">
    Visit site settings in the address bar and allow "Apps on Device".
</div>

<div class="mobile-wallet-adapter-embedded-modal-divider"><hr></div>
<div class="mobile-wallet-adapter-embedded-modal-footer">
    <div class="mobile-wallet-adapter-embedded-modal-details">
        <!-- Clickable header (label associated with the checkbox) -->
      	<label for="collapsible-1" class="mobile-wallet-adapter-embedded-modal-details-collapsible-header">
            <!-- Hidden checkbox to track state -->
            <input type="checkbox" id="collapsible-1" class="mobile-wallet-adapter-embedded-modal-details-collapsible-input">
            <span class="mobile-wallet-adapter-embedded-modal-details-collapsible-header-label">
              See details
            </span>
            <svg class="mobile-wallet-adapter-embedded-modal-details-collapsible-header-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <mask id="mask0_147_1382" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="0" y="0" width="24" height="24">
                <rect width="24" height="24" fill="#D9D9D9"/>
              </mask>
              <g mask="url(#mask0_147_1382)">
                <path d="M11.9999 17.0811C11.8506 17.0811 11.7087 17.0563 11.5741 17.0067C11.4395 16.957 11.3162 16.8762 11.2042 16.7643L6.57924 12.1393C6.36801 11.9281 6.26656 11.667 6.27489 11.3561C6.28322 11.0453 6.39301 10.7842 6.60424 10.573C6.81547 10.3618 7.08069 10.2561 7.39989 10.2561C7.71909 10.2561 7.9843 10.3618 8.19554 10.573L11.9999 14.3773L15.8292 10.548C16.0405 10.3368 16.3015 10.2353 16.6124 10.2436C16.9233 10.252 17.1843 10.3618 17.3955 10.573C17.6068 10.7842 17.7124 11.0494 17.7124 11.3686C17.7124 11.6878 17.6068 11.9531 17.3955 12.1643L12.7955 16.7643C12.6836 16.8762 12.5603 16.957 12.4257 17.0067C12.2911 17.0563 12.1492 17.0811 11.9999 17.0811Z" fill="black"/>
              </g>
            </svg>
      	</label>
        
        <!-- Content to show/hide -->
        <ul class="mobile-wallet-adapter-embedded-modal-details-collapsible-content">
            <li>{{PERMISSION_INSTRUCTION_DETAIL}}</li>
            <li>Allow "Apps on Device"</li>
        </ul>
    </div>
</div>
<div>
    <button data-modal-action id="mobile-wallet-adapter-launch-action" class="mobile-wallet-adapter-embedded-modal-launch-action">
        Got it
    </button>
</div>
`,f_=`
.mobile-wallet-adapter-embedded-modal-close {
    display: none;
}
.mobile-wallet-adapter-embedded-modal-content {
    text-align: center;
}
.mobile-wallet-adapter-embedded-modal-header {
    margin: 18px auto 30px auto;
    color: #7D9093;
    font-size: 1.0em;
    font-weight: 500;
}
.mobile-wallet-adapter-embedded-modal-title {
    margin: 18px 100px auto 100px;
    color: #000000;
    font-size: 2.75em;
    font-weight: 600;
}
.mobile-wallet-adapter-embedded-modal-subtitle {
    margin: 12px 60px 30px 60px;
    color: #7D9093;
    font-size: 1.25em;
    font-weight: 400;
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-header {
    display: flex;
    flex-direction: row;
  	justify-content: space-between;
    margin: 10px auto 10px auto;
    color: #000000;
    font-size: 1.5em;
    font-weight: 600;
    cursor: pointer; /* Show pointer on hover */
    transition: background 0.2s ease; /* Smooth background change */
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-header-icon {
  	transition: rotate 0.3s ease;
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-input {
  	display: none; /* Hide the checkbox */
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-content {
    margin: 0px auto 40px auto;
    max-height: 0px; /* Collapse content */
    overflow: hidden; /* Hide overflow when collapsed */
    transition: max-height 0.3s ease; /* Smooth transition */
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-content li {
    margin: 20px auto;
    color: #000000;
    font-size: 1.25em;
    font-weight: 400;
    text-align: left;
}
/* When checkbox is checked, show content */
.mobile-wallet-adapter-embedded-modal-details-collapsible-header:has(> input:checked) ~ .mobile-wallet-adapter-embedded-modal-details-collapsible-content {
  	max-height: 300px;
}
.mobile-wallet-adapter-embedded-modal-details-collapsible-header:has(> input:checked) > .mobile-wallet-adapter-embedded-modal-details-collapsible-header-icon {
  	rotate: 180deg;
}
.mobile-wallet-adapter-embedded-modal-launch-action {
    display: block;
    width: 100%;
    height: 56px;
    /*margin-top: 40px;*/
    font-size: 1.25em;
    /*line-height: 24px;*/
    /*letter-spacing: -1%;*/
    background: #000000;
    color: #FFFFFF;
    border-radius: 18px;
}
/* Smaller screens */
@media all and (max-width: 600px) {
    .mobile-wallet-adapter-embedded-modal-title {
        font-size: 1.75em;
        margin-right: 12px;
        margin-left: 12px;
    }
    .mobile-wallet-adapter-embedded-modal-subtitle {
        margin-right: 12px;
        margin-left: 12px;
    }
}
`,g_=class extends Re{contentStyles=I_;contentHtml=T_;async init(){super.init(),this.#e()}#e(){let t=this.dom?.getElementById("mobile-wallet-adapter-launch-action"),e=async()=>{t?.removeEventListener("click",e);try{await fetch("http://localhost")}catch{}this.close()};t?.addEventListener("click",e)}},T_=`
<div class="mobile-wallet-adapter-embedded-modal-title">Allow connections to your wallet</div>
<div id="mobile-wallet-adapter-local-launch-message" class="mobile-wallet-adapter-embedded-modal-subtitle">
    Tap "Allow" on the next screen
</div>
<svg class="mobile-wallet-adapter-embedded-modal-permission-prompt-mock" xmlns="http://www.w3.org/2000/svg" width="281" height="83" viewBox="0 0 281 83" fill="none">
    <rect width="281" height="83" rx="22" fill="#F0F3F5"/>
    <path d="M254.194 64L252.626 56.657H254.047L254.866 61.452L254.985 62.278H255.02L255.146 61.452L255.993 57.497H257.4L258.254 61.431L258.373 62.278H258.415L258.534 61.431L259.346 56.657H260.718L259.143 64H257.673L256.826 59.961L256.693 59.093H256.651L256.511 59.961L255.664 64H254.194Z" fill="black"/>
    <path d="M248.837 64.231C248.147 64.231 247.54 64.07 247.017 63.748C246.495 63.426 246.086 62.978 245.792 62.404C245.498 61.83 245.351 61.1673 245.351 60.416V60.241C245.351 59.4897 245.498 58.827 245.792 58.253C246.086 57.679 246.495 57.2333 247.017 56.916C247.54 56.594 248.147 56.433 248.837 56.433C249.528 56.433 250.135 56.594 250.657 56.916C251.18 57.2333 251.588 57.679 251.882 58.253C252.176 58.827 252.323 59.4897 252.323 60.241V60.416C252.323 61.1673 252.176 61.83 251.882 62.404C251.588 62.978 251.18 63.426 250.657 63.748C250.135 64.07 249.528 64.231 248.837 64.231ZM248.837 62.824C249.43 62.824 249.897 62.607 250.237 62.173C250.583 61.7343 250.755 61.1417 250.755 60.395V60.262C250.755 59.5107 250.583 58.918 250.237 58.484C249.897 58.05 249.43 57.833 248.837 57.833C248.249 57.833 247.783 58.05 247.437 58.484C247.092 58.918 246.919 59.5107 246.919 60.262V60.395C246.919 61.1417 247.092 61.7343 247.437 62.173C247.783 62.607 248.249 62.824 248.837 62.824Z" fill="black"/>
    <path d="M242.298 64.231C241.467 64.231 240.814 63.993 240.338 63.517C239.866 63.0364 239.631 62.3737 239.631 61.529V53.78H241.178V61.389C241.178 62.3317 241.591 62.803 242.417 62.803C242.65 62.803 242.865 62.7587 243.061 62.67C243.257 62.5814 243.464 62.4367 243.684 62.236L244.538 63.377C244.225 63.6664 243.884 63.881 243.516 64.021C243.152 64.161 242.746 64.231 242.298 64.231ZM237.51 55.061V53.78H240.611V55.061H237.51Z" fill="black"/>
    <path d="M234.463 64.231C233.633 64.231 232.979 63.993 232.503 63.517C232.032 63.0364 231.796 62.3737 231.796 61.529V53.78H233.343V61.389C233.343 62.3317 233.756 62.803 234.582 62.803C234.816 62.803 235.03 62.7587 235.226 62.67C235.422 62.5814 235.63 62.4367 235.849 62.236L236.703 63.377C236.391 63.6664 236.05 63.881 235.681 64.021C235.317 64.161 234.911 64.231 234.463 64.231ZM229.675 55.061V53.78H232.776V55.061H229.675Z" fill="black"/>
    <path d="M221.442 64L224.557 53.976H226.132L229.233 64H227.581L225.642 56.972L225.341 55.761H225.299L225.005 56.972L223.073 64H221.442ZM222.835 61.634L223.255 60.29H227.371L227.805 61.634H222.835Z" fill="black"/>
    <path d="M178.261 64L175.034 60.066V60.024L178.121 56.657H180.011L176.504 60.423V59.632L180.165 64H178.261ZM173.543 64V53.78H175.097V64H173.543Z" fill="#7D9093" fill-opacity="0.5"/>
    <path d="M169.306 64.224C168.588 64.224 167.958 64.0653 167.416 63.748C166.88 63.426 166.462 62.9803 166.163 62.411C165.865 61.837 165.715 61.1673 165.715 60.402V60.248C165.715 59.4873 165.862 58.8223 166.156 58.253C166.45 57.679 166.863 57.2333 167.395 56.916C167.927 56.594 168.546 56.433 169.25 56.433C169.978 56.433 170.59 56.6056 171.084 56.951C171.579 57.2917 171.955 57.777 172.211 58.407L170.874 58.995C170.72 58.6123 170.508 58.323 170.237 58.127C169.967 57.9263 169.633 57.826 169.236 57.826C168.63 57.826 168.149 58.0383 167.794 58.463C167.444 58.883 167.269 59.4616 167.269 60.199V60.465C167.269 61.1837 167.454 61.7577 167.822 62.187C168.196 62.6163 168.69 62.831 169.306 62.831C169.712 62.831 170.06 62.733 170.349 62.537C170.639 62.341 170.877 62.0423 171.063 61.641L172.379 62.285C172.188 62.6957 171.941 63.0457 171.637 63.335C171.334 63.6243 170.986 63.846 170.594 64C170.202 64.1493 169.773 64.224 169.306 64.224Z" fill="#7D9093" fill-opacity="0.5"/>
    <path d="M161.003 64.231C160.312 64.231 159.706 64.07 159.183 63.748C158.66 63.426 158.252 62.978 157.958 62.404C157.664 61.83 157.517 61.1673 157.517 60.416V60.241C157.517 59.4897 157.664 58.827 157.958 58.253C158.252 57.679 158.66 57.2333 159.183 56.916C159.706 56.594 160.312 56.433 161.003 56.433C161.694 56.433 162.3 56.594 162.823 56.916C163.346 57.2333 163.754 57.679 164.048 58.253C164.342 58.827 164.489 59.4897 164.489 60.241V60.416C164.489 61.1673 164.342 61.83 164.048 62.404C163.754 62.978 163.346 63.426 162.823 63.748C162.3 64.07 161.694 64.231 161.003 64.231ZM161.003 62.824C161.596 62.824 162.062 62.607 162.403 62.173C162.748 61.7343 162.921 61.1417 162.921 60.395V60.262C162.921 59.5107 162.748 58.918 162.403 58.484C162.062 58.05 161.596 57.833 161.003 57.833C160.415 57.833 159.948 58.05 159.603 58.484C159.258 58.918 159.085 59.5107 159.085 60.262V60.395C159.085 61.1417 159.258 61.7343 159.603 62.173C159.948 62.607 160.415 62.824 161.003 62.824Z" fill="#7D9093" fill-opacity="0.5"/>
    <path d="M154.463 64.231C153.633 64.231 152.979 63.993 152.503 63.517C152.032 63.0364 151.796 62.3737 151.796 61.529V53.78H153.343V61.389C153.343 62.3317 153.756 62.803 154.582 62.803C154.816 62.803 155.03 62.7587 155.226 62.67C155.422 62.5814 155.63 62.4367 155.849 62.236L156.703 63.377C156.391 63.6664 156.05 63.881 155.681 64.021C155.317 64.161 154.911 64.231 154.463 64.231ZM149.675 55.061V53.78H152.776V55.061H149.675Z" fill="#7D9093" fill-opacity="0.5"/>
    <path d="M142.24 64V53.976H145.544C146.421 53.976 147.112 54.1953 147.616 54.634C148.12 55.0726 148.372 55.6583 148.372 56.391V56.566C148.372 57.0886 148.246 57.5366 147.994 57.91C147.742 58.2833 147.38 58.5586 146.909 58.736V58.792C147.492 58.9226 147.947 59.2003 148.274 59.625C148.605 60.045 148.771 60.5606 148.771 61.172V61.361C148.771 61.893 148.645 62.3573 148.393 62.754C148.145 63.1506 147.795 63.4586 147.343 63.678C146.895 63.8926 146.365 64 145.754 64H142.24ZM143.794 62.656H145.572C146.085 62.656 146.482 62.5253 146.762 62.264C147.042 62.0026 147.182 61.6293 147.182 61.144V60.99C147.182 60.5046 147.037 60.1313 146.748 59.87C146.463 59.604 146.05 59.471 145.509 59.471H143.36V58.183H145.32C145.791 58.183 146.153 58.064 146.405 57.826C146.657 57.588 146.783 57.2496 146.783 56.811V56.685C146.783 56.2416 146.657 55.9033 146.405 55.67C146.157 55.4366 145.796 55.32 145.32 55.32H143.794V62.656Z" fill="#7D9093" fill-opacity="0.5"/>
    <rect x="18" y="17" width="246" height="7" rx="3.5" fill="#7D9093" fill-opacity="0.26"/>
    <rect x="18" y="33" width="82" height="7" rx="3.5" fill="#7D9093" fill-opacity="0.26"/>
</svg>
<div>
    <button data-modal-action id="mobile-wallet-adapter-launch-action" class="mobile-wallet-adapter-embedded-modal-launch-action">
        Continue to Allow
    </button>
</div>
`,I_=`
.mobile-wallet-adapter-embedded-modal-close {
    display: none;
}
.mobile-wallet-adapter-embedded-modal-content {
    text-align: center;
}
.mobile-wallet-adapter-embedded-modal-title {
    margin: 18px 100px auto 100px;
    color: #000000;
    font-size: 2.75em;
    font-weight: 600;
}
.mobile-wallet-adapter-embedded-modal-subtitle {
    margin: 20px 60px 40px 60px;
    color: #7D9093;
    font-size: 1.25em;
    font-weight: 400;
}
.mobile-wallet-adapter-embedded-modal-permission-prompt-mock {
    width: 90%;
    height: auto;
    margin: 0 auto 30px auto;
    display: block;
}
.mobile-wallet-adapter-embedded-modal-launch-action {
    display: block;
    width: 100%;
    height: 56px;
    font-size: 1.25em;
    background: #000000;
    color: #FFFFFF;
    border-radius: 18px;
}
/* Smaller screens */
@media all and (max-width: 600px) {
    .mobile-wallet-adapter-embedded-modal-title {
        font-size: 1.5em;
        margin-right: 12px;
        margin-left: 12px;
    }
    .mobile-wallet-adapter-embedded-modal-subtitle {
        margin-right: 12px;
        margin-left: 12px;
    }
}
`;function C_(){return typeof window<"u"&&window.isSecureContext&&typeof document<"u"&&/android/i.test(navigator.userAgent)}function w_(){return typeof window<"u"&&window.isSecureContext&&typeof document<"u"&&!/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)}function L_(t){return/(WebView|Version\/.+(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+)|; wv\).+(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+))/i.test(t)}function aa(t){return t.includes("Solana Mobile Web Shell")}function b_(){let t=typeof document<"u"&&document.referrer.startsWith("android-app://");if(typeof window>"u")return t;let e=window.matchMedia("(display-mode: standalone)").matches,n=window.matchMedia("(display-mode: fullscreen)").matches,a=window.matchMedia("(display-mode: minimal-ui)").matches;return t||e||n||a}async function ra(){if(!(typeof navigator<"u"&&aa(navigator.userAgent)))try{let t=await navigator.permissions.query({name:"loopback-network"});if(t.state==="granted")return;if(t.state==="denied"){let e=new m_;throw e.init(),e.open(),new m(p.ERROR_LOOPBACK_ACCESS_BLOCKED,"Local Network Access permission denied")}else if(t.state==="prompt"){let e=new g_;if(await new Promise((n,a)=>{e.addEventListener("close",r=>{r&&a(new m(p.ERROR_ASSOCIATION_CANCELLED,"Wallet connection cancelled by user",{event:r}))}),t.onchange=()=>{t.onchange=null,n(t.state)},e.init(),e.open()})==="granted"){let n=new O_;await new Promise((a,r)=>{n.addEventListener("close",o=>{o&&r(new m(p.ERROR_ASSOCIATION_CANCELLED,"Wallet connection cancelled by user",{event:o}))}),n.initWithCallback(async()=>{a(!0)}),n.open()});return}else return await ra()}throw new m(p.ERROR_LOOPBACK_ACCESS_BLOCKED,"Local Network Access permission unknown")}catch(t){if(t instanceof TypeError&&(t.message.includes("loopback-network")||t.message.includes("local-network-access")))return;throw t instanceof m?t:new m(p.ERROR_LOOPBACK_ACCESS_BLOCKED,t instanceof Error?t.message:"Local Network Access permission unknown")}}var oa="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZmlsbC1ydWxlPSJldmVub2RkIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik03IDIuNUgxN0MxNy44Mjg0IDIuNSAxOC41IDMuMTcxNTcgMTguNSA0VjIwQzE4LjUgMjAuODI4NCAxNy44Mjg0IDIxLjUgMTcgMjEuNUg3QzYuMTcxNTcgMjEuNSA1LjUgMjAuODI4NCA1LjUgMjBWNEM1LjUgMy4xNzE1NyA2LjE3MTU3IDIuNSA3IDIuNVpNMyA0QzMgMS43OTA4NiA0Ljc5MDg2IDAgNyAwSDE3QzE5LjIwOTEgMCAyMSAxLjc5MDg2IDIxIDRWMjBDMjEgMjIuMjA5MSAxOS4yMDkxIDI0IDE3IDI0SDdDNC43OTA4NiAyNCAzIDIyLjIwOTEgMyAyMFY0Wk0xMSA0LjYxNTM4QzEwLjQ0NzcgNC42MTUzOCAxMCA1LjA2MzEgMTAgNS42MTUzOFY2LjM4NDYyQzEwIDYuOTM2OSAxMC40NDc3IDcuMzg0NjIgMTEgNy4zODQ2MkgxM0MxMy41NTIzIDcuMzg0NjIgMTQgNi45MzY5IDE0IDYuMzg0NjJWNS42MTUzOEMxNCA1LjA2MzEgMTMuNTUyMyA0LjYxNTM4IDEzIDQuNjE1MzhIMTFaIiBmaWxsPSIjRENCOEZGIi8+Cjwvc3ZnPgo=",ia="Mobile Wallet Adapter",sa="Remote Mobile Wallet Adapter",ca=64,la=[B,F,me,Se],y_=3e4;function x(t){return t instanceof Error?t.message:"Unknown error"}var da=class{#e={};#n="1.0.0";#r=ia;#d="https://solanamobile.com/wallets";#_=oa;#s;#t;#a;#o=!1;#u=0;#c=[];#N;#R;#S;get version(){return this.#n}get name(){return this.#r}get url(){return this.#d}get icon(){return this.#_}get chains(){return this.#c}get features(){return{[Ye]:{version:"1.0.0",connect:this.#m},[Xe]:{version:"1.0.0",disconnect:this.#I},[je]:{version:"1.0.0",on:this.#g},[me]:{version:"1.0.0",signMessage:this.#b},[Se]:{version:"1.0.0",signIn:this.#y},...this.#R}}get accounts(){return this.#t?.accounts??[]}constructor(t){this.#a=t.authorizationCache,this.#s=t.appIdentity,this.#c=t.chains,this.#N=t.chainSelector,this.#S=t.onWalletNotFound,this.#R={[B]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signAndSendTransaction:this.#w},[F]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signTransaction:this.#L}}}get connected(){return!!this.#t}get isAuthorized(){return!!this.#t}get currentAuthorization(){return this.#t}get cachedAuthorizationResult(){return this.#a.get()}#g=(t,e)=>(this.#e[t]?.push(e)||(this.#e[t]=[e]),()=>this.#D(t,e));#i(t,...e){this.#e[t]?.forEach(n=>n.apply(null,e))}#D(t,e){this.#e[t]=this.#e[t]?.filter(n=>e!==n)}#m=async({silent:t}={})=>{if(this.#o||this.connected)return{accounts:this.accounts};this.#o=!0;try{if(t){let e=await this.#a.get();if(e)await this.#p(e.capabilities),await this.#A(e);else return{accounts:this.accounts}}else await this.#T()}catch(e){throw new Error(x(e),{cause:e})}finally{this.#o=!1}return{accounts:this.accounts}};#T=async t=>{try{let e=await this.#a.get();if(e)return this.#A(e),e;let n=await this.#N.select(this.#c);return await this.#l(async a=>{let[r,o]=await Promise.all([a.getCapabilities(),a.authorize({chain:n,identity:this.#s,sign_in_payload:t})]),i=this.#h(o.accounts),s={...o,accounts:i,chain:n,capabilities:r};return Promise.all([this.#p(r),this.#a.set(s),this.#A(s)]),s})}catch(e){throw new Error(x(e),{cause:e})}};#A=async t=>{let e=this.#t==null||this.#t?.accounts.length!==t.accounts.length||this.#t.accounts.some((n,a)=>n.address!==t.accounts[a].address);this.#t=t,e&&this.#i("change",{accounts:this.accounts})};#p=async t=>{let e=t.features.includes("solana:signTransactions"),n=t.supports_sign_and_send_transactions,a=B in this.features!==n||F in this.features!==e;this.#R={...(n||!n&&!e)&&{[B]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signAndSendTransaction:this.#w}},...e&&{[F]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signTransaction:this.#L}}},a&&this.#i("change",{features:this.features})};#E=async(t,e,n)=>{try{let[a,r]=await Promise.all([this.#t?.capabilities??await t.getCapabilities(),t.authorize({auth_token:e,identity:this.#s,chain:n})]),o=this.#h(r.accounts),i={...r,accounts:o,chain:n,capabilities:a};Promise.all([this.#a.set(i),this.#A(i)])}catch(a){throw this.#I(),new Error(x(a),{cause:a})}};#I=async()=>{this.#a.clear(),this.#o=!1,this.#u++,this.#t=void 0,this.#i("change",{accounts:this.accounts})};#l=async t=>{let e=this.#t?.wallet_uri_base,n=e?{baseUri:e}:void 0,a=this.#u,r=new d_;try{let o=!0,i,s=await Promise.race([ra().then(async()=>{r.init();let{wallet:c,close:l}=await Zt(n);o=!1,r.addEventListener("close",R=>{R&&l()}),r.open();let d=await t(await c);return r.close(),l(),d}),new Promise((c,l)=>{i=setTimeout(()=>{o&&l(new m(p.ERROR_ASSOCIATION_CANCELLED,"Wallet connection timed out",{event:void 0}))},y_)})]);return clearTimeout(i),s}catch(o){throw r.close(),this.#u!==a&&await new Promise(()=>{}),o instanceof Error&&o.name==="SolanaMobileWalletAdapterError"&&o.code==="ERROR_WALLET_NOT_FOUND"&&await this.#S(this),o}};#O=()=>{if(!this.#t)throw new Error("Wallet not connected");return{authToken:this.#t.auth_token,chain:this.#t.chain}};#h=t=>t.map(e=>{let n=D(e.address);return{address:j.encode(n),publicKey:n,label:e.label,icon:e.icon,chains:e.chains??this.#c,features:e.features??la}});#f=async t=>{let{authToken:e,chain:n}=this.#O();try{let a=t.map(r=>G(r));return await this.#l(async r=>(await this.#E(r,e,n),(await r.signTransactions({payloads:a})).signed_payloads.map(D)))}catch(a){throw new Error(x(a),{cause:a})}};#C=async(t,e)=>{let{authToken:n,chain:a}=this.#O();try{return await this.#l(async r=>{let[o]=await Promise.all([r.getCapabilities(),this.#E(r,n,a)]);if(o.supports_sign_and_send_transactions){let i=G(t);return(await r.signAndSendTransactions({...e,payloads:[i]})).signatures.map(D)[0]}else throw new Error("connected wallet does not support signAndSendTransaction")})}catch(r){throw new Error(x(r),{cause:r})}};#w=async(...t)=>{let e=[];for(let n of t){let a=await this.#C(n.transaction,n.options);e.push({signature:a})}return e};#L=async(...t)=>(await this.#f(t.map(({transaction:e})=>e))).map(e=>({signedTransaction:e}));#b=async(...t)=>{let{authToken:e,chain:n}=this.#O(),a=t.map(({account:o})=>G(new Uint8Array(o.publicKey))),r=t.map(({message:o})=>G(o));try{return await this.#l(async o=>(await this.#E(o,e,n),(await o.signMessages({addresses:a,payloads:r})).signed_payloads.map(D).map(i=>({signedMessage:i,signature:i.slice(-ca)}))))}catch(o){throw new Error(x(o),{cause:o})}};#y=async(...t)=>{let e=[];if(t.length>1)for(let n of t)e.push(await this.#v(n));else return[await this.#v(t[0])];return e};#v=async t=>{this.#o=!0;try{let e=await this.#T({...t,domain:t?.domain??window.location.host});if(!e.sign_in_result)throw new Error("Sign in failed, no sign in result returned by wallet");let n=e.sign_in_result.address,a=e.accounts.find(r=>r.address==n);return{account:{...a??{address:j.encode(D(n))},publicKey:D(n),chains:a?.chains??this.#c,features:a?.features??e.capabilities.features},signedMessage:D(e.sign_in_result.signed_message),signature:D(e.sign_in_result.signature)}}catch(e){throw new Error(x(e),{cause:e})}finally{this.#o=!1}}},_a=class{#e={};#n="1.0.0";#r=sa;#d="https://solanamobile.com/wallets";#_=oa;#s;#t;#a;#o=!1;#u=0;#c=[];#N;#R;#S;#g;#i;get version(){return this.#n}get name(){return this.#r}get url(){return this.#d}get icon(){return this.#_}get chains(){return this.#c}get features(){return{[Ye]:{version:"1.0.0",connect:this.#A},[Xe]:{version:"1.0.0",disconnect:this.#O},[je]:{version:"1.0.0",on:this.#D},[me]:{version:"1.0.0",signMessage:this.#v},[Se]:{version:"1.0.0",signIn:this.#M},...this.#R}}get accounts(){return this.#t?.accounts??[]}constructor(t){this.#a=t.authorizationCache,this.#s=t.appIdentity,this.#c=t.chains,this.#N=t.chainSelector,this.#g=t.remoteHostAuthority,this.#S=t.onWalletNotFound,this.#R={[B]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signAndSendTransaction:this.#b},[F]:{version:"1.0.0",supportedTransactionVersions:["legacy",0],signTransaction:this.#y}}}get connected(){return!!this.#i&&!!this.#t}get isAuthorized(){return!!this.#t}get currentAuthorization(){return this.#t}get cachedAuthorizationResult(){return this.#a.get()}#D=(t,e)=>(this.#e[t]?.push(e)||(this.#e[t]=[e]),()=>this.#T(t,e));#m(t,...e){this.#e[t]?.forEach(n=>n.apply(null,e))}#T(t,e){this.#e[t]=this.#e[t]?.filter(n=>e!==n)}#A=async(t={})=>{if(this.#o||this.connected)return{accounts:this.accounts};this.#o=!0;try{await this.#p()}catch(e){throw new Error(x(e),{cause:e})}finally{this.#o=!1}return{accounts:this.accounts}};#p=async t=>{try{let e=await this.#a.get();if(e)return this.#E(e),e;this.#i&&(this.#i=void 0);let n=await this.#N.select(this.#c);return await this.#h(async a=>{let[r,o]=await Promise.all([a.getCapabilities(),a.authorize({chain:n,identity:this.#s,sign_in_payload:t})]),i=this.#C(o.accounts),s={...o,accounts:i,chain:n,capabilities:r};return Promise.all([this.#I(r),this.#a.set(s),this.#E(s)]),s})}catch(e){throw new Error(x(e),{cause:e})}};#E=async t=>{let e=this.#t==null||this.#t?.accounts.length!==t.accounts.length||this.#t.accounts.some((n,a)=>n.address!==t.accounts[a].address);this.#t=t,e&&this.#m("change",{accounts:this.accounts})};#I=async t=>{let e=t.features.includes("solana:signTransactions"),n=t.supports_sign_and_send_transactions||t.features.includes("solana:signAndSendTransaction"),a=B in this.features!==n||F in this.features!==e;this.#R={...n&&{[B]:{version:"1.0.0",supportedTransactionVersions:t.supported_transaction_versions,signAndSendTransaction:this.#b}},...e&&{[F]:{version:"1.0.0",supportedTransactionVersions:t.supported_transaction_versions,signTransaction:this.#y}}},a&&this.#m("change",{features:this.features})};#l=async(t,e,n)=>{try{let[a,r]=await Promise.all([this.#t?.capabilities??await t.getCapabilities(),t.authorize({auth_token:e,identity:this.#s,chain:n})]),o=this.#C(r.accounts),i={...r,accounts:o,chain:n,capabilities:a};Promise.all([this.#a.set(i),this.#E(i)])}catch(a){throw this.#O(),new Error(x(a),{cause:a})}};#O=async()=>{this.#i?.close(),this.#a.clear(),this.#o=!1,this.#u++,this.#t=void 0,this.#i=void 0,this.#m("change",{accounts:this.accounts})};#h=async t=>{let e=this.#t?.wallet_uri_base,n={...e?{baseUri:e}:void 0,remoteHostAuthority:this.#g},a=this.#u,r=new E_;if(this.#i)return t(this.#i.wallet);try{r.init(),r.open();let{associationUrl:o,close:i,wallet:s}=await Qt(n),c=r.addEventListener("close",l=>{l&&i()});return r.populateQRCode(o.toString()),this.#i={close:i,wallet:await s},c(),r.close(),await t(this.#i.wallet)}catch(o){throw r.close(),this.#u!==a&&await new Promise(()=>{}),o instanceof Error&&o.name==="SolanaMobileWalletAdapterError"&&o.code==="ERROR_WALLET_NOT_FOUND"&&await this.#S(this),o}};#f=()=>{if(!this.#t)throw new Error("Wallet not connected");return{authToken:this.#t.auth_token,chain:this.#t.chain}};#C=t=>t.map(e=>{let n=D(e.address);return{address:j.encode(n),publicKey:n,label:e.label,icon:e.icon,chains:e.chains??this.#c,features:e.features??la}});#w=async t=>{let{authToken:e,chain:n}=this.#f();try{return await this.#h(async a=>(await this.#l(a,e,n),(await a.signTransactions({payloads:t.map(G)})).signed_payloads.map(D)))}catch(a){throw new Error(x(a),{cause:a})}};#L=async(t,e)=>{let{authToken:n,chain:a}=this.#f();try{return await this.#h(async r=>{let[o]=await Promise.all([r.getCapabilities(),this.#l(r,n,a)]);if(o.supports_sign_and_send_transactions)return(await r.signAndSendTransactions({...e,payloads:[G(t)]})).signatures.map(D)[0];throw new Error("connected wallet does not support signAndSendTransaction")})}catch(r){throw new Error(x(r),{cause:r})}};#b=async(...t)=>{let e=[];for(let n of t){let a=await this.#L(n.transaction,n.options);e.push({signature:a})}return e};#y=async(...t)=>(await this.#w(t.map(({transaction:e})=>e))).map(e=>({signedTransaction:e}));#v=async(...t)=>{let{authToken:e,chain:n}=this.#f(),a=t.map(({account:o})=>G(new Uint8Array(o.publicKey))),r=t.map(({message:o})=>G(o));try{return await this.#h(async o=>(await this.#l(o,e,n),(await o.signMessages({addresses:a,payloads:r})).signed_payloads.map(D).map(i=>({signedMessage:i,signature:i.slice(-ca)}))))}catch(o){throw new Error(x(o),{cause:o})}};#M=async(...t)=>{let e=[];if(t.length>1)for(let n of t)e.push(await this.#x(n));else return[await this.#x(t[0])];return e};#x=async t=>{this.#o=!0;try{let e=await this.#p({...t,domain:t?.domain??window.location.host});if(!e.sign_in_result)throw new Error("Sign in failed, no sign in result returned by wallet");let n=e.sign_in_result.address,a=e.accounts.find(r=>r.address==n);return{account:{...a??{address:j.encode(D(n))},publicKey:D(n),chains:a?.chains??this.#c,features:a?.features??e.capabilities.features},signedMessage:D(e.sign_in_result.signed_message),signature:D(e.sign_in_result.signature)}}catch(e){throw new Error(x(e),{cause:e})}finally{this.#o=!1}}};function v_(t){if(typeof window>"u"){console.warn("MWA not registered: no window object");return}if(!window.isSecureContext){console.warn("MWA not registered: secure context required (https)");return}let e=navigator.userAgent;C_()&&(!L_(e)||aa(e))?pt(new da(t)):w_()&&t.remoteHostAuthority!==void 0&&pt(new _a({...t,remoteHostAuthority:t.remoteHostAuthority}))}var D_="To use mobile wallet adapter, you must have a compatible mobile wallet application installed on your device.",x_="This browser appears to be incompatible with mobile wallet adapter. Open this page in a compatible mobile browser app and try again.",M_=class extends Re{contentStyles=P_;contentHtml=U_;initWithError(t){super.init(),this.populateError(t)}populateError(t){let e=this.dom?.getElementById("mobile-wallet-adapter-error-message"),n=this.dom?.getElementById("mobile-wallet-adapter-error-action");if(e){if(t.name==="SolanaMobileWalletAdapterError")switch(t.code){case"ERROR_WALLET_NOT_FOUND":e.innerHTML=D_,n&&n.addEventListener("click",()=>{window.location.href="https://solanamobile.com/wallets"});return;case"ERROR_BROWSER_NOT_SUPPORTED":e.innerHTML=x_,n&&(n.style.display="none");return}e.innerHTML=`An unexpected error occurred: ${t.message}`}else console.log("Failed to locate error dialog element")}},U_=`
<svg class="mobile-wallet-adapter-embedded-modal-error-icon" xmlns="http://www.w3.org/2000/svg" height="50px" viewBox="0 -960 960 960" width="50px" fill="#000000"><path d="M 280,-80 Q 197,-80 138.5,-138.5 80,-197 80,-280 80,-363 138.5,-421.5 197,-480 280,-480 q 83,0 141.5,58.5 58.5,58.5 58.5,141.5 0,83 -58.5,141.5 Q 363,-80 280,-80 Z M 824,-120 568,-376 Q 556,-389 542.5,-402.5 529,-416 516,-428 q 38,-24 61,-64 23,-40 23,-88 0,-75 -52.5,-127.5 Q 495,-760 420,-760 345,-760 292.5,-707.5 240,-655 240,-580 q 0,6 0.5,11.5 0.5,5.5 1.5,11.5 -18,2 -39.5,8 -21.5,6 -38.5,14 -2,-11 -3,-22 -1,-11 -1,-23 0,-109 75.5,-184.5 Q 311,-840 420,-840 q 109,0 184.5,75.5 75.5,75.5 75.5,184.5 0,43 -13.5,81.5 Q 653,-460 629,-428 l 251,252 z m -615,-61 71,-71 70,71 29,-28 -71,-71 71,-71 -28,-28 -71,71 -71,-71 -28,28 71,71 -71,71 z"/></svg>
<div class="mobile-wallet-adapter-embedded-modal-title">We can't find a wallet.</div>
<div id="mobile-wallet-adapter-error-message" class="mobile-wallet-adapter-embedded-modal-subtitle"></div>
<div>
    <button data-error-action id="mobile-wallet-adapter-error-action" class="mobile-wallet-adapter-embedded-modal-error-action">
        Find a wallet
    </button>
</div>
`,P_=`
.mobile-wallet-adapter-embedded-modal-content {
    text-align: center;
}

.mobile-wallet-adapter-embedded-modal-error-icon {
    margin-top: 24px;
}

.mobile-wallet-adapter-embedded-modal-title {
    margin: 18px 100px auto 100px;
    color: #000000;
    font-size: 2.75em;
    font-weight: 600;
}

.mobile-wallet-adapter-embedded-modal-subtitle {
    margin: 30px 60px 40px 60px;
    color: #000000;
    font-size: 1.25em;
    font-weight: 400;
}

.mobile-wallet-adapter-embedded-modal-error-action {
    display: block;
    width: 100%;
    height: 56px;
    /*margin-top: 40px;*/
    font-size: 1.25em;
    /*line-height: 24px;*/
    /*letter-spacing: -1%;*/
    background: #000000;
    color: #FFFFFF;
    border-radius: 18px;
}

/* Smaller screens */
@media all and (max-width: 600px) {
    .mobile-wallet-adapter-embedded-modal-title {
        font-size: 1.5em;
        margin-right: 12px;
        margin-left: 12px;
    }
    .mobile-wallet-adapter-embedded-modal-subtitle {
        margin-right: 12px;
        margin-left: 12px;
    }
}
`;async function ua(){if(typeof window<"u"){let t=window.navigator.userAgent.toLowerCase(),e=new M_;t.includes("wv")?e.initWithError({name:"SolanaMobileWalletAdapterError",code:"ERROR_BROWSER_NOT_SUPPORTED",message:""}):e.initWithError({name:"SolanaMobileWalletAdapterError",code:"ERROR_WALLET_NOT_FOUND",message:""}),e.open()}}function B_(){return async()=>{ua()}}var ft="SolanaMobileWalletAdapterDefaultAuthorizationCache";function F_(){let t;try{t=window.localStorage}catch{}return{async clear(){if(t)try{t.removeItem(ft)}catch{}},async get(){if(t)try{let e=JSON.parse(t.getItem(ft));if(e&&e.accounts){let n=e.accounts.map(a=>({...a,publicKey:"publicKey"in a?new Uint8Array(Object.values(a.publicKey)):j.decode(a.address)}));return{...e,accounts:n}}else return e||void 0}catch{}},async set(e){if(t)try{t.setItem(ft,JSON.stringify(e))}catch{}}}}function $_(){return{async select(t){return t.length===1?t[0]:t.includes(We)?We:t[0]}}}return pa(k_);})();
globalThis.SolanaMobileWalletStandard=SolanaMobileWalletStandard;

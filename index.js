!function(e){function t(t){for(var n,o,c=t[0],a=t[1],i=0,u=[];i<c.length;i++)o=c[i],Object.prototype.hasOwnProperty.call(r,o)&&r[o]&&u.push(r[o][0]),r[o]=0;for(n in a)Object.prototype.hasOwnProperty.call(a,n)&&(e[n]=a[n]);for(_&&_(t);u.length;)u.shift()()}var n={},r={0:0};var o={};var c={3:function(){return{"./index.js":{__wbg_new_59cb74e423758ede:function(){return n[1].exports.__wbg_new_59cb74e423758ede()},__wbg_stack_558ba5917b466edd:function(e,t){return n[1].exports.__wbg_stack_558ba5917b466edd(e,t)},__wbg_error_4bb6c2a97407129a:function(e,t){return n[1].exports.__wbg_error_4bb6c2a97407129a(e,t)},__wbindgen_object_drop_ref:function(e){return n[1].exports.__wbindgen_object_drop_ref(e)},__wbg_self_e70540c4956ad879:function(){return n[1].exports.__wbg_self_e70540c4956ad879()},__wbg_require_9edeecb69c9dc31c:function(e,t){return n[1].exports.__wbg_require_9edeecb69c9dc31c(e,t)},__wbg_crypto_58b0c631995fea92:function(e){return n[1].exports.__wbg_crypto_58b0c631995fea92(e)},__wbindgen_is_undefined:function(e){return n[1].exports.__wbindgen_is_undefined(e)},__wbg_getRandomValues_532ec62d8e780edc:function(e){return n[1].exports.__wbg_getRandomValues_532ec62d8e780edc(e)},__wbg_getRandomValues_40ceff860009fa55:function(e,t,r){return n[1].exports.__wbg_getRandomValues_40ceff860009fa55(e,t,r)},__wbg_randomFillSync_eabbc18af655bfbe:function(e,t,r){return n[1].exports.__wbg_randomFillSync_eabbc18af655bfbe(e,t,r)},__wbindgen_throw:function(e,t){return n[1].exports.__wbindgen_throw(e,t)}}}}};function a(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,a),r.l=!0,r.exports}a.e=function(e){var t=[],n=r[e];if(0!==n)if(n)t.push(n[2]);else{var i=new Promise((function(t,o){n=r[e]=[t,o]}));t.push(n[2]=i);var u,s=document.createElement("script");s.charset="utf-8",s.timeout=120,a.nc&&s.setAttribute("nonce",a.nc),s.src=function(e){return a.p+""+({}[e]||e)+".js"}(e);var _=new Error;u=function(t){s.onerror=s.onload=null,clearTimeout(f);var n=r[e];if(0!==n){if(n){var o=t&&("load"===t.type?"missing":t.type),c=t&&t.target&&t.target.src;_.message="Loading chunk "+e+" failed.\n("+o+": "+c+")",_.name="ChunkLoadError",_.type=o,_.request=c,n[1](_)}r[e]=void 0}};var f=setTimeout((function(){u({type:"timeout",target:s})}),12e4);s.onerror=s.onload=u,document.head.appendChild(s)}return({1:[3]}[e]||[]).forEach((function(e){var n=o[e];if(n)t.push(n);else{var r,i=c[e](),u=fetch(a.p+""+{3:"928121878b14795e720d"}[e]+".module.wasm");if(i instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(u),i]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(u,i);else{r=u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,i)}))}t.push(o[e]=r.then((function(t){return a.w[e]=(t.instance||t).exports})))}})),Promise.all(t)},a.m=e,a.c=n,a.d=function(e,t,n){a.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},a.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,t){if(1&t&&(e=a(e)),8&t)return e;if(4&t&&"object"==typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)a.d(n,r,function(t){return e[t]}.bind(null,r));return n},a.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return a.d(t,"a",t),t},a.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},a.p="",a.oe=function(e){throw console.error(e),e},a.w={};var i=window.webpackJsonp=window.webpackJsonp||[],u=i.push.bind(i);i.push=t,i=i.slice();for(var s=0;s<i.length;s++)t(i[s]);var _=u;a(a.s=0)}([function(e,t,n){n.e(1).then(n.bind(null,1)).then(e=>{s(e)}).catch(console.error);const r=document.getElementById("tetris-mk-canvas"),o=document.getElementById("score"),c=document.getElementById("next_shape"),a=0,i=1,u=2,s=e=>{const{Board:t}=e;let n=t.new(10,20);const s=()=>{r.textContent=n.render(),o.textContent=n.get_score(),c.textContent=n.get_next_shape_type(),n.tick()||(alert("Game over! Your score: "+n.get_score()),n=t.new(10,20)),setTimeout(()=>{requestAnimationFrame(s)},500)};document.onkeydown=e=>{e||window.event;switch(e.key){case"Down":case"ArrowDown":case"j":n.move_shape(u);break;case"Left":case"ArrowLeft":case"h":n.move_shape(a);break;case"Right":case"ArrowRight":case"l":n.move_shape(i);break;case"r":n.rotate();break;default:return}r.textContent=n.render()},document.getElementById("down").onclick=()=>n.move_shape(u),document.getElementById("left").onclick=()=>n.move_shape(a),document.getElementById("right").onclick=()=>n.move_shape(i),document.getElementById("rotate").onclick=()=>n.rotate(),requestAnimationFrame(s)}}]);
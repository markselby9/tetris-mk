(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(t,e,n){"use strict";n.r(e),n.d(e,"init_panic_hook",(function(){return a})),n.d(e,"Cell",(function(){return _})),n.d(e,"ShapeType",(function(){return v})),n.d(e,"Direction",(function(){return O})),n.d(e,"Board",(function(){return j})),n.d(e,"Shape",(function(){return T})),n.d(e,"__wbg_new_59cb74e423758ede",(function(){return x})),n.d(e,"__wbg_stack_558ba5917b466edd",(function(){return E})),n.d(e,"__wbg_error_4bb6c2a97407129a",(function(){return S})),n.d(e,"__wbindgen_object_drop_ref",(function(){return z})),n.d(e,"__wbg_self_e70540c4956ad879",(function(){return D})),n.d(e,"__wbg_require_9edeecb69c9dc31c",(function(){return k})),n.d(e,"__wbg_crypto_58b0c631995fea92",(function(){return P})),n.d(e,"__wbindgen_is_undefined",(function(){return A})),n.d(e,"__wbg_getRandomValues_532ec62d8e780edc",(function(){return R})),n.d(e,"__wbg_randomFillSync_eabbc18af655bfbe",(function(){return N})),n.d(e,"__wbg_getRandomValues_40ceff860009fa55",(function(){return F})),n.d(e,"__wbindgen_throw",(function(){return L}));var r=n(3);let o=null;function i(){return null!==o&&o.buffer===r.q.buffer||(o=new Int32Array(r.q.buffer)),o}let u=new("undefined"==typeof TextDecoder?n(2).TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});u.decode();let c=null;function f(){return null!==c&&c.buffer===r.q.buffer||(c=new Uint8Array(r.q.buffer)),c}function s(t,e){return u.decode(f().subarray(t,t+e))}function a(){r.p()}const l=new Array(32);l.fill(void 0),l.push(void 0,null,!0,!1);let p=l.length;function d(t){p===l.length&&l.push(l.length+1);const e=p;return p=l[e],l[e]=t,e}function y(t){return l[t]}let g=0;let h=new("undefined"==typeof TextEncoder?n(2).TextEncoder:TextEncoder)("utf-8");const b="function"==typeof h.encodeInto?function(t,e){return h.encodeInto(t,e)}:function(t,e){const n=h.encode(t);return e.set(n),{read:t.length,written:n.length}};function m(t){const e=y(t);return function(t){t<36||(l[t]=p,p=t)}(t),e}function w(t,e){return f().subarray(t/1,t/1+e)}const _=Object.freeze({Empty:0,Running:1,Placed:2}),v=Object.freeze({Square:0,S:1,Z:2,T:3,L:4,Line:5,MirroredL:6,Random:7}),O=Object.freeze({Left:0,Right:1,Down:2});class j{static __wrap(t){const e=Object.create(j.prototype);return e.ptr=t,e}free(){const t=this.ptr;this.ptr=0,r.a(t)}static new(t,e){const n=r.l(t,e);return j.__wrap(n)}get_width(){return r.j(this.ptr)>>>0}get_height(){return r.g(this.ptr)>>>0}get_score(){return r.i(this.ptr)}get_next_shape_type(){return r.h(this.ptr)}render(){r.m(8,this.ptr);const t=i(),e=s(t[2],t[3]).slice();return r.d(t[2],1*t[3]),e}tick(){return 0!==r.o(this.ptr)}move_shape(t){r.k(this.ptr,t)}rotate(){r.n(this.ptr)}}class T{free(){const t=this.ptr;this.ptr=0,r.b(t)}}const x=function(){return d(new Error)},E=function(t,e){const n=function(t){let e=t.length,n=r.e(e);const o=f();let i=0;for(;i<e;i++){const e=t.charCodeAt(i);if(e>127)break;o[n+i]=e}if(i!==e){0!==i&&(t=t.slice(i)),n=r.f(n,e,e=i+3*t.length);const o=f().subarray(n+i,n+e);i+=b(t,o).written}return g=i,n}(y(e).stack),o=g;i()[t/4+0]=n,i()[t/4+1]=o},S=function(t,e){const n=s(t,e).slice();r.d(t,1*e),console.error(n)},z=function(t){m(t)},D=function(){try{return d(self.self)}catch(t){!function(t){r.c(d(t))}(t)}},k=function(t,e){return d(n(7)(s(t,e)))},P=function(t){return d(y(t).crypto)},A=function(t){return void 0===y(t)},R=function(t){return d(y(t).getRandomValues)},N=function(t,e,n){y(t).randomFillSync(w(e,n))},F=function(t,e,n){y(t).getRandomValues(w(e,n))},L=function(t,e){throw new Error(s(t,e))}},function(t,e,n){(function(t){var r=Object.getOwnPropertyDescriptors||function(t){for(var e=Object.keys(t),n={},r=0;r<e.length;r++)n[e[r]]=Object.getOwnPropertyDescriptor(t,e[r]);return n},o=/%[sdj%]/g;e.format=function(t){if(!b(t)){for(var e=[],n=0;n<arguments.length;n++)e.push(c(arguments[n]));return e.join(" ")}n=1;for(var r=arguments,i=r.length,u=String(t).replace(o,(function(t){if("%%"===t)return"%";if(n>=i)return t;switch(t){case"%s":return String(r[n++]);case"%d":return Number(r[n++]);case"%j":try{return JSON.stringify(r[n++])}catch(t){return"[Circular]"}default:return t}})),f=r[n];n<i;f=r[++n])g(f)||!_(f)?u+=" "+f:u+=" "+c(f);return u},e.deprecate=function(n,r){if(void 0!==t&&!0===t.noDeprecation)return n;if(void 0===t)return function(){return e.deprecate(n,r).apply(this,arguments)};var o=!1;return function(){if(!o){if(t.throwDeprecation)throw new Error(r);t.traceDeprecation?console.trace(r):console.error(r),o=!0}return n.apply(this,arguments)}};var i,u={};function c(t,n){var r={seen:[],stylize:s};return arguments.length>=3&&(r.depth=arguments[2]),arguments.length>=4&&(r.colors=arguments[3]),y(n)?r.showHidden=n:n&&e._extend(r,n),m(r.showHidden)&&(r.showHidden=!1),m(r.depth)&&(r.depth=2),m(r.colors)&&(r.colors=!1),m(r.customInspect)&&(r.customInspect=!0),r.colors&&(r.stylize=f),a(r,t,r.depth)}function f(t,e){var n=c.styles[e];return n?"["+c.colors[n][0]+"m"+t+"["+c.colors[n][1]+"m":t}function s(t,e){return t}function a(t,n,r){if(t.customInspect&&n&&j(n.inspect)&&n.inspect!==e.inspect&&(!n.constructor||n.constructor.prototype!==n)){var o=n.inspect(r,t);return b(o)||(o=a(t,o,r)),o}var i=function(t,e){if(m(e))return t.stylize("undefined","undefined");if(b(e)){var n="'"+JSON.stringify(e).replace(/^"|"$/g,"").replace(/'/g,"\\'").replace(/\\"/g,'"')+"'";return t.stylize(n,"string")}if(h(e))return t.stylize(""+e,"number");if(y(e))return t.stylize(""+e,"boolean");if(g(e))return t.stylize("null","null")}(t,n);if(i)return i;var u=Object.keys(n),c=function(t){var e={};return t.forEach((function(t,n){e[t]=!0})),e}(u);if(t.showHidden&&(u=Object.getOwnPropertyNames(n)),O(n)&&(u.indexOf("message")>=0||u.indexOf("description")>=0))return l(n);if(0===u.length){if(j(n)){var f=n.name?": "+n.name:"";return t.stylize("[Function"+f+"]","special")}if(w(n))return t.stylize(RegExp.prototype.toString.call(n),"regexp");if(v(n))return t.stylize(Date.prototype.toString.call(n),"date");if(O(n))return l(n)}var s,_="",T=!1,x=["{","}"];(d(n)&&(T=!0,x=["[","]"]),j(n))&&(_=" [Function"+(n.name?": "+n.name:"")+"]");return w(n)&&(_=" "+RegExp.prototype.toString.call(n)),v(n)&&(_=" "+Date.prototype.toUTCString.call(n)),O(n)&&(_=" "+l(n)),0!==u.length||T&&0!=n.length?r<0?w(n)?t.stylize(RegExp.prototype.toString.call(n),"regexp"):t.stylize("[Object]","special"):(t.seen.push(n),s=T?function(t,e,n,r,o){for(var i=[],u=0,c=e.length;u<c;++u)z(e,String(u))?i.push(p(t,e,n,r,String(u),!0)):i.push("");return o.forEach((function(o){o.match(/^\d+$/)||i.push(p(t,e,n,r,o,!0))})),i}(t,n,r,c,u):u.map((function(e){return p(t,n,r,c,e,T)})),t.seen.pop(),function(t,e,n){if(t.reduce((function(t,e){return e.indexOf("\n")>=0&&0,t+e.replace(/\u001b\[\d\d?m/g,"").length+1}),0)>60)return n[0]+(""===e?"":e+"\n ")+" "+t.join(",\n  ")+" "+n[1];return n[0]+e+" "+t.join(", ")+" "+n[1]}(s,_,x)):x[0]+_+x[1]}function l(t){return"["+Error.prototype.toString.call(t)+"]"}function p(t,e,n,r,o,i){var u,c,f;if((f=Object.getOwnPropertyDescriptor(e,o)||{value:e[o]}).get?c=f.set?t.stylize("[Getter/Setter]","special"):t.stylize("[Getter]","special"):f.set&&(c=t.stylize("[Setter]","special")),z(r,o)||(u="["+o+"]"),c||(t.seen.indexOf(f.value)<0?(c=g(n)?a(t,f.value,null):a(t,f.value,n-1)).indexOf("\n")>-1&&(c=i?c.split("\n").map((function(t){return"  "+t})).join("\n").substr(2):"\n"+c.split("\n").map((function(t){return"   "+t})).join("\n")):c=t.stylize("[Circular]","special")),m(u)){if(i&&o.match(/^\d+$/))return c;(u=JSON.stringify(""+o)).match(/^"([a-zA-Z_][a-zA-Z_0-9]*)"$/)?(u=u.substr(1,u.length-2),u=t.stylize(u,"name")):(u=u.replace(/'/g,"\\'").replace(/\\"/g,'"').replace(/(^"|"$)/g,"'"),u=t.stylize(u,"string"))}return u+": "+c}function d(t){return Array.isArray(t)}function y(t){return"boolean"==typeof t}function g(t){return null===t}function h(t){return"number"==typeof t}function b(t){return"string"==typeof t}function m(t){return void 0===t}function w(t){return _(t)&&"[object RegExp]"===T(t)}function _(t){return"object"==typeof t&&null!==t}function v(t){return _(t)&&"[object Date]"===T(t)}function O(t){return _(t)&&("[object Error]"===T(t)||t instanceof Error)}function j(t){return"function"==typeof t}function T(t){return Object.prototype.toString.call(t)}function x(t){return t<10?"0"+t.toString(10):t.toString(10)}e.debuglog=function(n){if(m(i)&&(i=t.env.NODE_DEBUG||""),n=n.toUpperCase(),!u[n])if(new RegExp("\\b"+n+"\\b","i").test(i)){var r=t.pid;u[n]=function(){var t=e.format.apply(e,arguments);console.error("%s %d: %s",n,r,t)}}else u[n]=function(){};return u[n]},e.inspect=c,c.colors={bold:[1,22],italic:[3,23],underline:[4,24],inverse:[7,27],white:[37,39],grey:[90,39],black:[30,39],blue:[34,39],cyan:[36,39],green:[32,39],magenta:[35,39],red:[31,39],yellow:[33,39]},c.styles={special:"cyan",number:"yellow",boolean:"yellow",undefined:"grey",null:"bold",string:"green",date:"magenta",regexp:"red"},e.isArray=d,e.isBoolean=y,e.isNull=g,e.isNullOrUndefined=function(t){return null==t},e.isNumber=h,e.isString=b,e.isSymbol=function(t){return"symbol"==typeof t},e.isUndefined=m,e.isRegExp=w,e.isObject=_,e.isDate=v,e.isError=O,e.isFunction=j,e.isPrimitive=function(t){return null===t||"boolean"==typeof t||"number"==typeof t||"string"==typeof t||"symbol"==typeof t||void 0===t},e.isBuffer=n(5);var E=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];function S(){var t=new Date,e=[x(t.getHours()),x(t.getMinutes()),x(t.getSeconds())].join(":");return[t.getDate(),E[t.getMonth()],e].join(" ")}function z(t,e){return Object.prototype.hasOwnProperty.call(t,e)}e.log=function(){console.log("%s - %s",S(),e.format.apply(e,arguments))},e.inherits=n(6),e._extend=function(t,e){if(!e||!_(e))return t;for(var n=Object.keys(e),r=n.length;r--;)t[n[r]]=e[n[r]];return t};var D="undefined"!=typeof Symbol?Symbol("util.promisify.custom"):void 0;function k(t,e){if(!t){var n=new Error("Promise was rejected with a falsy value");n.reason=t,t=n}return e(t)}e.promisify=function(t){if("function"!=typeof t)throw new TypeError('The "original" argument must be of type Function');if(D&&t[D]){var e;if("function"!=typeof(e=t[D]))throw new TypeError('The "util.promisify.custom" argument must be of type Function');return Object.defineProperty(e,D,{value:e,enumerable:!1,writable:!1,configurable:!0}),e}function e(){for(var e,n,r=new Promise((function(t,r){e=t,n=r})),o=[],i=0;i<arguments.length;i++)o.push(arguments[i]);o.push((function(t,r){t?n(t):e(r)}));try{t.apply(this,o)}catch(t){n(t)}return r}return Object.setPrototypeOf(e,Object.getPrototypeOf(t)),D&&Object.defineProperty(e,D,{value:e,enumerable:!1,writable:!1,configurable:!0}),Object.defineProperties(e,r(t))},e.promisify.custom=D,e.callbackify=function(e){if("function"!=typeof e)throw new TypeError('The "original" argument must be of type Function');function n(){for(var n=[],r=0;r<arguments.length;r++)n.push(arguments[r]);var o=n.pop();if("function"!=typeof o)throw new TypeError("The last argument must be of type Function");var i=this,u=function(){return o.apply(i,arguments)};e.apply(this,n).then((function(e){t.nextTick(u,null,e)}),(function(e){t.nextTick(k,e,u)}))}return Object.setPrototypeOf(n,Object.getPrototypeOf(e)),Object.defineProperties(n,r(e)),n}}).call(this,n(4))},function(t,e,n){"use strict";var r=n.w[t.i];t.exports=r;n(1);r.r()},function(t,e){var n,r,o=t.exports={};function i(){throw new Error("setTimeout has not been defined")}function u(){throw new Error("clearTimeout has not been defined")}function c(t){if(n===setTimeout)return setTimeout(t,0);if((n===i||!n)&&setTimeout)return n=setTimeout,setTimeout(t,0);try{return n(t,0)}catch(e){try{return n.call(null,t,0)}catch(e){return n.call(this,t,0)}}}!function(){try{n="function"==typeof setTimeout?setTimeout:i}catch(t){n=i}try{r="function"==typeof clearTimeout?clearTimeout:u}catch(t){r=u}}();var f,s=[],a=!1,l=-1;function p(){a&&f&&(a=!1,f.length?s=f.concat(s):l=-1,s.length&&d())}function d(){if(!a){var t=c(p);a=!0;for(var e=s.length;e;){for(f=s,s=[];++l<e;)f&&f[l].run();l=-1,e=s.length}f=null,a=!1,function(t){if(r===clearTimeout)return clearTimeout(t);if((r===u||!r)&&clearTimeout)return r=clearTimeout,clearTimeout(t);try{r(t)}catch(e){try{return r.call(null,t)}catch(e){return r.call(this,t)}}}(t)}}function y(t,e){this.fun=t,this.array=e}function g(){}o.nextTick=function(t){var e=new Array(arguments.length-1);if(arguments.length>1)for(var n=1;n<arguments.length;n++)e[n-1]=arguments[n];s.push(new y(t,e)),1!==s.length||a||c(d)},y.prototype.run=function(){this.fun.apply(null,this.array)},o.title="browser",o.browser=!0,o.env={},o.argv=[],o.version="",o.versions={},o.on=g,o.addListener=g,o.once=g,o.off=g,o.removeListener=g,o.removeAllListeners=g,o.emit=g,o.prependListener=g,o.prependOnceListener=g,o.listeners=function(t){return[]},o.binding=function(t){throw new Error("process.binding is not supported")},o.cwd=function(){return"/"},o.chdir=function(t){throw new Error("process.chdir is not supported")},o.umask=function(){return 0}},function(t,e){t.exports=function(t){return t&&"object"==typeof t&&"function"==typeof t.copy&&"function"==typeof t.fill&&"function"==typeof t.readUInt8}},function(t,e){"function"==typeof Object.create?t.exports=function(t,e){t.super_=e,t.prototype=Object.create(e.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}})}:t.exports=function(t,e){t.super_=e;var n=function(){};n.prototype=e.prototype,t.prototype=new n,t.prototype.constructor=t}},function(t,e){function n(t){var e=new Error("Cannot find module '"+t+"'");throw e.code="MODULE_NOT_FOUND",e}n.keys=function(){return[]},n.resolve=n,t.exports=n,n.id=7}]]);
(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(n,t,e){"use strict";e.r(t),e.d(t,"init_panic_hook",(function(){return _})),e.d(t,"Cell",(function(){return m})),e.d(t,"ShapeType",(function(){return k})),e.d(t,"Direction",(function(){return v})),e.d(t,"Board",(function(){return O})),e.d(t,"Shape",(function(){return R})),e.d(t,"__wbg_new_59cb74e423758ede",(function(){return j})),e.d(t,"__wbg_stack_558ba5917b466edd",(function(){return E})),e.d(t,"__wbg_error_4bb6c2a97407129a",(function(){return S})),e.d(t,"__wbindgen_object_drop_ref",(function(){return D})),e.d(t,"__wbg_self_e70540c4956ad879",(function(){return L})),e.d(t,"__wbg_require_9edeecb69c9dc31c",(function(){return T})),e.d(t,"__wbg_crypto_58b0c631995fea92",(function(){return x})),e.d(t,"__wbindgen_is_undefined",(function(){return A})),e.d(t,"__wbg_getRandomValues_532ec62d8e780edc",(function(){return V})),e.d(t,"__wbg_randomFillSync_eabbc18af655bfbe",(function(){return q})),e.d(t,"__wbg_getRandomValues_40ceff860009fa55",(function(){return z})),e.d(t,"__wbindgen_string_new",(function(){return C})),e.d(t,"__widl_f_log_1_",(function(){return F})),e.d(t,"__wbindgen_throw",(function(){return I}));var r=e(2);let o=null;function u(){return null!==o&&o.buffer===r.p.buffer||(o=new Int32Array(r.p.buffer)),o}let c=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});c.decode();let i=null;function f(){return null!==i&&i.buffer===r.p.buffer||(i=new Uint8Array(r.p.buffer)),i}function d(n,t){return c.decode(f().subarray(n,n+t))}function _(){r.o()}const s=new Array(32);s.fill(void 0),s.push(void 0,null,!0,!1);let a=s.length;function l(n){a===s.length&&s.push(s.length+1);const t=a;return a=s[t],s[t]=n,t}function b(n){return s[n]}let w=0,p=new TextEncoder("utf-8");const g="function"==typeof p.encodeInto?function(n,t){return p.encodeInto(n,t)}:function(n,t){const e=p.encode(n);return t.set(e),{read:n.length,written:e.length}};function h(n){const t=b(n);return function(n){n<36||(s[n]=a,a=n)}(n),t}function y(n,t){return f().subarray(n/1,n/1+t)}const m=Object.freeze({Empty:0,Running:1,Placed:2}),k=Object.freeze({Square:0,S:1,Z:2,T:3,L:4,Line:5,MirroredL:6,Random:7}),v=Object.freeze({Left:0,Right:1,Down:2});class O{static __wrap(n){const t=Object.create(O.prototype);return t.ptr=n,t}free(){const n=this.ptr;this.ptr=0,r.a(n)}static new(n,t){const e=r.k(n,t);return O.__wrap(e)}get_width(){return r.i(this.ptr)>>>0}get_height(){return r.g(this.ptr)>>>0}get_score(){return r.h(this.ptr)}render(){r.l(8,this.ptr);const n=u(),t=d(n[2],n[3]).slice();return r.d(n[2],1*n[3]),t}tick(){return 0!==r.n(this.ptr)}move_shape(n){r.j(this.ptr,n)}rotate(){r.m(this.ptr)}}class R{free(){const n=this.ptr;this.ptr=0,r.b(n)}}const j=function(){return l(new Error)},E=function(n,t){const e=function(n){let t=n.length,e=r.e(t);const o=f();let u=0;for(;u<t;u++){const t=n.charCodeAt(u);if(t>127)break;o[e+u]=t}if(u!==t){0!==u&&(n=n.slice(u)),e=r.f(e,t,t=u+3*n.length);const o=f().subarray(e+u,e+t);u+=g(n,o).written}return w=u,e}(b(t).stack),o=w;u()[n/4+0]=e,u()[n/4+1]=o},S=function(n,t){const e=d(n,t).slice();r.d(n,1*t),console.error(e)},D=function(n){h(n)},L=function(){try{return l(self.self)}catch(n){!function(n){r.c(l(n))}(n)}},T=function(n,t){return l(e(3)(d(n,t)))},x=function(n){return l(b(n).crypto)},A=function(n){return void 0===b(n)},V=function(n){return l(b(n).getRandomValues)},q=function(n,t,e){b(n).randomFillSync(y(t,e))},z=function(n,t,e){b(n).getRandomValues(y(t,e))},C=function(n,t){return l(d(n,t))},F=function(n){console.log(b(n))},I=function(n,t){throw new Error(d(n,t))}},function(n,t,e){"use strict";var r=e.w[n.i];n.exports=r;e(1);r.q()},function(n,t){function e(n){var t=new Error("Cannot find module '"+n+"'");throw t.code="MODULE_NOT_FOUND",t}e.keys=function(){return[]},e.resolve=e,n.exports=e,e.id=3}]]);
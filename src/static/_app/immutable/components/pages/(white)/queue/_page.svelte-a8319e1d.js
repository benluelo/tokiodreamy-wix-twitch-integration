import{S as O,i as Q,s as R,e as S,b as E,t as y,d as L,f as k,h,H as T,o as U,g as P,k as v,q as N,l as b,m as g,r as D,G as p,B as V,n as I,I as M,w as j,x as F,y as J,z as K,a as w,c as x,u as B}from"../../../../chunks/index-1b64e03c.js";import{r as W,b as X}from"../../../../chunks/client-7ab338e8.js";import{C as Y,L as Z}from"../../../../chunks/LineItem-b2efe33e.js";function C(f,e,t){const l=f.slice();return l[2]=e[t],l}function q(f,e,t){const l=f.slice();return l[5]=e[t],l}function ee(f){let e,t;return{c(){e=v("div"),t=N("Loading...")},l(l){e=b(l,"DIV",{});var n=g(e);t=D(n,"Loading..."),n.forEach(h)},m(l,n){E(l,e,n),p(e,t)},p:V,i:V,o:V,d(l){l&&h(e)}}}function te(f){let e,t,l,n;const s=[re,le],c=[];function a(r,o){return r[1].ordered_breaks.length===0?0:1}return e=a(f),t=c[e]=s[e](f),{c(){t.c(),l=S()},l(r){t.l(r),l=S()},m(r,o){c[e].m(r,o),E(r,l,o),n=!0},p(r,o){let _=e;e=a(r),e===_?c[e].p(r,o):(P(),y(c[_],1,1,()=>{c[_]=null}),L(),t=c[e],t?t.p(r,o):(t=c[e]=s[e](r),t.c()),k(t,1),t.m(l.parentNode,l))},i(r){n||(k(t),n=!0)},o(r){y(t),n=!1},d(r){c[e].d(r),r&&h(l)}}}function le(f){let e,t,l,n=f[1].ordered_breaks,s=[];for(let a=0;a<n.length;a+=1)s[a]=H(C(f,n,a));const c=a=>y(s[a],1,1,()=>{s[a]=null});return{c(){e=v("div"),t=v("div");for(let a=0;a<s.length;a+=1)s[a].c();this.h()},l(a){e=b(a,"DIV",{class:!0});var r=g(e);t=b(r,"DIV",{class:!0});var o=g(t);for(let _=0;_<s.length;_+=1)s[_].l(o);o.forEach(h),r.forEach(h),this.h()},h(){I(t,"class","flex flex-col gap-y-2"),I(e,"class","max-w-3xl m-auto")},m(a,r){E(a,e,r),p(e,t);for(let o=0;o<s.length;o+=1)s[o].m(t,null);l=!0},p(a,r){if(r&2){n=a[1].ordered_breaks;let o;for(o=0;o<n.length;o+=1){const _=C(a,n,o);s[o]?(s[o].p(_,r),k(s[o],1)):(s[o]=H(_),s[o].c(),k(s[o],1),s[o].m(t,null))}for(P(),o=n.length;o<s.length;o+=1)c(o);L()}},i(a){if(!l){for(let r=0;r<n.length;r+=1)k(s[r]);l=!0}},o(a){s=s.filter(Boolean);for(let r=0;r<s.length;r+=1)y(s[r]);l=!1},d(a){a&&h(e),M(s,a)}}}function re(f){let e,t;return{c(){e=v("div"),t=N("no breaks lol")},l(l){e=b(l,"DIV",{});var n=g(e);t=D(n,"no breaks lol"),n.forEach(h)},m(l,n){E(l,e,n),p(e,t)},p:V,i:V,o:V,d(l){l&&h(e)}}}function oe(f){let e,t,l=f[2].twitch_username+"",n,s,c,a,r,o,_=f[2].order_id+"",u,d;return{c(){e=v("span"),t=v("div"),n=N(l),s=w(),c=v("div"),a=w(),r=v("span"),o=N("#"),u=N(_),d=w(),this.h()},l(i){e=b(i,"SPAN",{slot:!0});var m=g(e);t=b(m,"DIV",{class:!0});var $=g(t);n=D($,l),s=x($),c=b($,"DIV",{class:!0}),g(c).forEach(h),a=x($),r=b($,"SPAN",{class:!0});var A=g(r);o=D(A,"#"),u=D(A,_),A.forEach(h),$.forEach(h),d=x(m),m.forEach(h),this.h()},h(){I(c,"class","grow"),I(r,"class","font-mono"),I(t,"class","flex"),I(e,"slot","header")},m(i,m){E(i,e,m),p(e,t),p(t,n),p(t,s),p(t,c),p(t,a),p(t,r),p(r,o),p(r,u),p(e,d)},p(i,m){m&2&&l!==(l=i[2].twitch_username+"")&&B(n,l),m&2&&_!==(_=i[2].order_id+"")&&B(u,_)},d(i){i&&h(e)}}}function z(f){let e,t,l=f[2].order.buyerNote+"",n;return{c(){e=v("div"),t=N("Note: "),n=N(l),this.h()},l(s){e=b(s,"DIV",{class:!0});var c=g(e);t=D(c,"Note: "),n=D(c,l),c.forEach(h),this.h()},h(){I(e,"class","font-semi-bold")},m(s,c){E(s,e,c),p(e,t),p(e,n)},p(s,c){c&2&&l!==(l=s[2].order.buyerNote+"")&&B(n,l)},d(s){s&&h(e)}}}function G(f){let e,t;return e=new Z({props:{lineItem:f[5]}}),{c(){j(e.$$.fragment)},l(l){F(e.$$.fragment,l)},m(l,n){J(e,l,n),t=!0},p(l,n){const s={};n&2&&(s.lineItem=l[5]),e.$set(s)},i(l){t||(k(e.$$.fragment,l),t=!0)},o(l){y(e.$$.fragment,l),t=!1},d(l){K(e,l)}}}function ne(f){let e,t,l,n,s,c,a=f[2].order.buyerNote&&z(f),r=f[2].order.lineItems,o=[];for(let u=0;u<r.length;u+=1)o[u]=G(q(f,r,u));const _=u=>y(o[u],1,1,()=>{o[u]=null});return{c(){e=v("span"),t=v("div"),a&&a.c(),l=w(),n=v("div");for(let u=0;u<o.length;u+=1)o[u].c();s=w(),this.h()},l(u){e=b(u,"SPAN",{slot:!0});var d=g(e);t=b(d,"DIV",{class:!0});var i=g(t);a&&a.l(i),l=x(i),n=b(i,"DIV",{class:!0});var m=g(n);for(let $=0;$<o.length;$+=1)o[$].l(m);m.forEach(h),i.forEach(h),s=x(d),d.forEach(h),this.h()},h(){I(n,"class","flex flex-wrap gap-2"),I(t,"class","flex flex-col gap-y-2 p-2"),I(e,"slot","content")},m(u,d){E(u,e,d),p(e,t),a&&a.m(t,null),p(t,l),p(t,n);for(let i=0;i<o.length;i+=1)o[i].m(n,null);p(e,s),c=!0},p(u,d){if(u[2].order.buyerNote?a?a.p(u,d):(a=z(u),a.c(),a.m(t,l)):a&&(a.d(1),a=null),d&2){r=u[2].order.lineItems;let i;for(i=0;i<r.length;i+=1){const m=q(u,r,i);o[i]?(o[i].p(m,d),k(o[i],1)):(o[i]=G(m),o[i].c(),k(o[i],1),o[i].m(n,null))}for(P(),i=r.length;i<o.length;i+=1)_(i);L()}},i(u){if(!c){for(let d=0;d<r.length;d+=1)k(o[d]);c=!0}},o(u){o=o.filter(Boolean);for(let d=0;d<o.length;d+=1)y(o[d]);c=!1},d(u){u&&h(e),a&&a.d(),M(o,u)}}}function H(f){let e,t;return e=new Y({props:{$$slots:{content:[ne],header:[oe]},$$scope:{ctx:f}}}),{c(){j(e.$$.fragment)},l(l){F(e.$$.fragment,l)},m(l,n){J(e,l,n),t=!0},p(l,n){const s={};n&258&&(s.$$scope={dirty:n,ctx:l}),e.$set(s)},i(l){t||(k(e.$$.fragment,l),t=!0)},o(l){y(e.$$.fragment,l),t=!1},d(l){K(e,l)}}}function se(f){let e,t,l,n;const s=[te,ee],c=[];function a(r,o){return r[0]?0:1}return e=a(f),t=c[e]=s[e](f),{c(){t.c(),l=S()},l(r){t.l(r),l=S()},m(r,o){c[e].m(r,o),E(r,l,o),n=!0},p(r,[o]){let _=e;e=a(r),e===_?c[e].p(r,o):(P(),y(c[_],1,1,()=>{c[_]=null}),L(),t=c[e],t?t.p(r,o):(t=c[e]=s[e](r),t.c()),k(t,1),t.m(l.parentNode,l))},i(r){n||(k(t),n=!0)},o(r){y(t),n=!1},d(r){c[e].d(r),r&&h(l)}}}function ae(f,e,t){let l;T(f,X,s=>t(1,l=s));let n=!1;return U(async()=>{t(0,n=!0),W()}),[n,l]}class ue extends O{constructor(e){super(),Q(this,e,ae,se,R,{})}}export{ue as default};
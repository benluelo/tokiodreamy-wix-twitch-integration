function $(){}function G(t,e){for(const n in e)t[n]=e[n];return t}function J(t){return!!t&&(typeof t=="object"||typeof t=="function")&&typeof t.then=="function"}function L(t){return t()}function M(){return Object.create(null)}function g(t){t.forEach(L)}function O(t){return typeof t=="function"}function at(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}function K(t){return Object.keys(t).length===0}function P(t,...e){if(t==null)return $;const n=t.subscribe(...e);return n.unsubscribe?()=>n.unsubscribe():n}function ft(t){let e;return P(t,n=>e=n)(),e}function _t(t,e,n){t.$$.on_destroy.push(P(e,n))}function dt(t,e,n,r){if(t){const c=q(t,e,n,r);return t[0](c)}}function q(t,e,n,r){return t[1]&&r?G(n.ctx.slice(),t[1](r(e))):n.ctx}function ht(t,e,n,r){if(t[2]&&r){const c=t[2](r(n));if(e.dirty===void 0)return c;if(typeof c=="object"){const s=[],i=Math.max(e.dirty.length,c.length);for(let o=0;o<i;o+=1)s[o]=e.dirty[o]|c[o];return s}return e.dirty|c}return e.dirty}function mt(t,e,n,r,c,s){if(c){const i=q(e,n,r,s);t.p(i,c)}}function pt(t){if(t.ctx.length>32){const e=[],n=t.ctx.length/32;for(let r=0;r<n;r++)e[r]=-1;return e}return-1}function yt(t){const e={};for(const n in t)e[n]=!0;return e}let w=!1;function Q(){w=!0}function R(){w=!1}function U(t,e,n,r){for(;t<e;){const c=t+(e-t>>1);n(c)<=r?t=c+1:e=c}return t}function W(t){if(t.hydrate_init)return;t.hydrate_init=!0;let e=t.childNodes;if(t.nodeName==="HEAD"){const u=[];for(let l=0;l<e.length;l++){const f=e[l];f.claim_order!==void 0&&u.push(f)}e=u}const n=new Int32Array(e.length+1),r=new Int32Array(e.length);n[0]=-1;let c=0;for(let u=0;u<e.length;u++){const l=e[u].claim_order,f=(c>0&&e[n[c]].claim_order<=l?c+1:U(1,c,d=>e[n[d]].claim_order,l))-1;r[u]=n[f]+1;const a=f+1;n[a]=u,c=Math.max(a,c)}const s=[],i=[];let o=e.length-1;for(let u=n[c]+1;u!=0;u=r[u-1]){for(s.push(e[u-1]);o>=u;o--)i.push(e[o]);o--}for(;o>=0;o--)i.push(e[o]);s.reverse(),i.sort((u,l)=>u.claim_order-l.claim_order);for(let u=0,l=0;u<i.length;u++){for(;l<s.length&&i[u].claim_order>=s[l].claim_order;)l++;const f=l<s.length?s[l]:null;t.insertBefore(i[u],f)}}function V(t,e){if(w){for(W(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentNode!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;e!==t.actual_end_child?(e.claim_order!==void 0||e.parentNode!==t)&&t.insertBefore(e,t.actual_end_child):t.actual_end_child=e.nextSibling}else(e.parentNode!==t||e.nextSibling!==null)&&t.appendChild(e)}function gt(t,e,n){w&&!n?V(t,e):(e.parentNode!==t||e.nextSibling!=n)&&t.insertBefore(e,n||null)}function X(t){t.parentNode&&t.parentNode.removeChild(t)}function bt(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}function Y(t){return document.createElement(t)}function Z(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function N(t){return document.createTextNode(t)}function xt(){return N(" ")}function $t(){return N("")}function wt(t,e,n,r){return t.addEventListener(e,n,r),()=>t.removeEventListener(e,n,r)}function vt(t,e,n){n==null?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function tt(t){return Array.from(t.childNodes)}function et(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function z(t,e,n,r,c=!1){et(t);const s=(()=>{for(let i=t.claim_info.last_index;i<t.length;i++){const o=t[i];if(e(o)){const u=n(o);return u===void 0?t.splice(i,1):t[i]=u,c||(t.claim_info.last_index=i),o}}for(let i=t.claim_info.last_index-1;i>=0;i--){const o=t[i];if(e(o)){const u=n(o);return u===void 0?t.splice(i,1):t[i]=u,c?u===void 0&&t.claim_info.last_index--:t.claim_info.last_index=i,o}}return r()})();return s.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,s}function D(t,e,n,r){return z(t,c=>c.nodeName===e,c=>{const s=[];for(let i=0;i<c.attributes.length;i++){const o=c.attributes[i];n[o.name]||s.push(o.name)}s.forEach(i=>c.removeAttribute(i))},()=>r(e))}function kt(t,e,n){return D(t,e,n,Y)}function Et(t,e,n){return D(t,e,n,Z)}function nt(t,e){return z(t,n=>n.nodeType===3,n=>{const r=""+e;if(n.data.startsWith(r)){if(n.data.length!==r.length)return n.splitText(r.length)}else n.data=r},()=>N(e),!0)}function Nt(t){return nt(t," ")}function St(t,e){e=""+e,t.wholeText!==e&&(t.data=e)}function jt(t,e){t.value=e??""}function At(t,e,n,r){n===null?t.style.removeProperty(e):t.style.setProperty(e,n,r?"important":"")}function Ct(t,e){return new t(e)}let y;function _(t){y=t}function S(){if(!y)throw new Error("Function called outside component initialization");return y}function Mt(t){S().$$.on_mount.push(t)}function Tt(t){S().$$.after_update.push(t)}const p=[],T=[],b=[],B=[],F=Promise.resolve();let k=!1;function H(){k||(k=!0,F.then(j))}function Bt(){return H(),F}function E(t){b.push(t)}const v=new Set;let m=0;function j(){if(m!==0)return;const t=y;do{try{for(;m<p.length;){const e=p[m];m++,_(e),rt(e.$$)}}catch(e){throw p.length=0,m=0,e}for(_(null),p.length=0,m=0;T.length;)T.pop()();for(let e=0;e<b.length;e+=1){const n=b[e];v.has(n)||(v.add(n),n())}b.length=0}while(p.length);for(;B.length;)B.pop()();k=!1,v.clear(),_(t)}function rt(t){if(t.fragment!==null){t.update(),g(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(E)}}const x=new Set;let h;function ct(){h={r:0,c:[],p:h}}function it(){h.r||g(h.c),h=h.p}function I(t,e){t&&t.i&&(x.delete(t),t.i(e))}function ut(t,e,n,r){if(t&&t.o){if(x.has(t))return;x.add(t),h.c.push(()=>{x.delete(t),r&&(n&&t.d(1),r())}),t.o(e)}else r&&r()}function Lt(t,e){const n=e.token={};function r(c,s,i,o){if(e.token!==n)return;e.resolved=o;let u=e.ctx;i!==void 0&&(u=u.slice(),u[i]=o);const l=c&&(e.current=c)(u);let f=!1;e.block&&(e.blocks?e.blocks.forEach((a,d)=>{d!==s&&a&&(ct(),ut(a,1,1,()=>{e.blocks[d]===a&&(e.blocks[d]=null)}),it())}):e.block.d(1),l.c(),I(l,1),l.m(e.mount(),e.anchor),f=!0),e.block=l,e.blocks&&(e.blocks[s]=l),f&&j()}if(J(t)){const c=S();if(t.then(s=>{_(c),r(e.then,1,e.value,s),_(null)},s=>{if(_(c),r(e.catch,2,e.error,s),_(null),!e.hasCatch)throw s}),e.current!==e.pending)return r(e.pending,0),!0}else{if(e.current!==e.then)return r(e.then,1,e.value,t),!0;e.resolved=t}}function Ot(t,e,n){const r=e.slice(),{resolved:c}=t;t.current===t.then&&(r[t.value]=c),t.current===t.catch&&(r[t.error]=c),t.block.p(r,n)}function Pt(t){t&&t.c()}function qt(t,e){t&&t.l(e)}function lt(t,e,n,r){const{fragment:c,after_update:s}=t.$$;c&&c.m(e,n),r||E(()=>{const i=t.$$.on_mount.map(L).filter(O);t.$$.on_destroy?t.$$.on_destroy.push(...i):g(i),t.$$.on_mount=[]}),s.forEach(E)}function st(t,e){const n=t.$$;n.fragment!==null&&(g(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function ot(t,e){t.$$.dirty[0]===-1&&(p.push(t),H(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function zt(t,e,n,r,c,s,i,o=[-1]){const u=y;_(t);const l=t.$$={fragment:null,ctx:[],props:s,update:$,not_equal:c,bound:M(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(u?u.$$.context:[])),callbacks:M(),dirty:o,skip_bound:!1,root:e.target||u.$$.root};i&&i(l.root);let f=!1;if(l.ctx=n?n(t,e.props||{},(a,d,...A)=>{const C=A.length?A[0]:d;return l.ctx&&c(l.ctx[a],l.ctx[a]=C)&&(!l.skip_bound&&l.bound[a]&&l.bound[a](C),f&&ot(t,a)),d}):[],l.update(),f=!0,g(l.before_update),l.fragment=r?r(l.ctx):!1,e.target){if(e.hydrate){Q();const a=tt(e.target);l.fragment&&l.fragment.l(a),a.forEach(X)}else l.fragment&&l.fragment.c();e.intro&&I(t.$$.fragment),lt(t,e.target,e.anchor,e.customElement),R(),j()}_(u)}class Dt{$destroy(){st(this,1),this.$destroy=$}$on(e,n){if(!O(n))return $;const r=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return r.push(n),()=>{const c=r.indexOf(n);c!==-1&&r.splice(c,1)}}$set(e){this.$$set&&!K(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}export{Bt as A,$ as B,dt as C,mt as D,pt as E,ht as F,V as G,_t as H,bt as I,ft as J,wt as K,O as L,jt as M,g as N,P as O,Lt as P,Ot as Q,Z as R,Dt as S,Et as T,yt as U,xt as a,gt as b,Nt as c,it as d,$t as e,I as f,ct as g,X as h,zt as i,Tt as j,Y as k,kt as l,tt as m,vt as n,Mt as o,At as p,N as q,nt as r,at as s,ut as t,St as u,Ct as v,Pt as w,qt as x,lt as y,st as z};
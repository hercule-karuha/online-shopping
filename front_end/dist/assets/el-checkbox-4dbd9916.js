import{W as U,X as M,aF as N,K as w,d as g,V as T,g as X,B as Y,M as Z,r as z,af as I,ag as ke,an as K,J as L,Y as _,L as D,o as C,C as j,m as ee,i as A,j as x,a as i,w as E,c as y,aQ as $,aT as F,U as P,F as ge,x as ne,t as ae,q as G,a9 as le,N as O,n as pe,H as xe,I as Ce,S as ye,ao as Se,a2 as Le,a3 as te}from"./index-43945a83.js";import{a2 as H,T as q,R as oe,a3 as se,a4 as Be,a5 as Ie,a6 as we,a7 as Ee,n as ie,U as V,h as $e,o as R,l as ue,p as W,a1 as re}from"./request-d304319d.js";import{v as de,j as Fe,w as Ve,i as Ne}from"./el-popper-a281cd70.js";function Te(e){return e}function ze(e,n,l){switch(l.length){case 0:return e.call(n);case 1:return e.call(n,l[0]);case 2:return e.call(n,l[0],l[1]);case 3:return e.call(n,l[0],l[1],l[2])}return e.apply(n,l)}var Ge=800,De=16,Pe=Date.now;function Oe(e){var n=0,l=0;return function(){var a=Pe(),t=De-(a-l);if(l=a,t>0){if(++n>=Ge)return arguments[0]}else n=0;return e.apply(void 0,arguments)}}function Re(e){return function(){return e}}var Ue=H?function(e,n){return H(e,"toString",{configurable:!0,enumerable:!1,value:Re(n),writable:!0})}:Te;const Me=Ue;var Ke=Oe(Me);const Ae=Ke;var J=Math.max;function He(e,n,l){return n=J(n===void 0?e.length-1:n,0),function(){for(var a=arguments,t=-1,d=J(a.length-n,0),c=Array(d);++t<d;)c[t]=a[n+t];t=-1;for(var o=Array(n+1);++t<n;)o[t]=a[t];return o[n]=l(c),ze(e,this,o)}}var Q=q?q.isConcatSpreadable:void 0;function qe(e){return oe(e)||de(e)||!!(Q&&e&&e[Q])}function ce(e,n,l,a,t){var d=-1,c=e.length;for(l||(l=qe),t||(t=[]);++d<c;){var o=e[d];n>0&&l(o)?n>1?ce(o,n-1,l,a,t):Fe(t,o):a||(t[t.length]=o)}return t}function We(e){var n=e==null?0:e.length;return n?ce(e,1):[]}function Je(e){return Ae(He(e,void 0,We),e+"")}function Qe(e,n){return e!=null&&n in Object(e)}function Xe(e,n,l){n=se(n,e);for(var a=-1,t=n.length,d=!1;++a<t;){var c=Be(n[a]);if(!(d=e!=null&&l(e,c)))break;e=e[c]}return d||++a!=t?d:(t=e==null?0:e.length,!!t&&Ve(t)&&Ie(c,t)&&(oe(e)||de(e)))}function Ye(e,n){return e!=null&&Xe(e,n,Qe)}function Ze(e,n,l){for(var a=-1,t=n.length,d={};++a<t;){var c=n[a],o=we(e,c);l(o,c)&&Ee(d,se(c,e),o)}return d}function _e(e,n){return Ze(e,n,function(l,a){return Ye(e,a)})}var je=Je(function(e,n){return e==null?{}:_e(e,n)});const en=je,be={modelValue:{type:[Number,String,Boolean],default:void 0},label:{type:[String,Boolean,Number,Object]},indeterminate:Boolean,disabled:Boolean,checked:Boolean,name:{type:String,default:void 0},trueLabel:{type:[String,Number],default:void 0},falseLabel:{type:[String,Number],default:void 0},id:{type:String,default:void 0},controls:{type:String,default:void 0},border:Boolean,size:ie,tabindex:[String,Number],validateEvent:{type:Boolean,default:!0}},ve={[V]:e=>U(e)||M(e)||N(e),change:e=>U(e)||M(e)||N(e)},B=Symbol("checkboxGroupContextKey"),nn=({model:e,isChecked:n})=>{const l=w(B,void 0),a=g(()=>{var d,c;const o=(d=l==null?void 0:l.max)==null?void 0:d.value,m=(c=l==null?void 0:l.min)==null?void 0:c.value;return!T(o)&&e.value.length>=o&&!n.value||!T(m)&&e.value.length<=m&&n.value});return{isDisabled:$e(g(()=>(l==null?void 0:l.disabled.value)||a.value)),isLimitDisabled:a}},an=(e,{model:n,isLimitExceeded:l,hasOwnLabel:a,isDisabled:t,isLabeledByFormItem:d})=>{const c=w(B,void 0),{formItem:o}=R(),{emit:m}=X();function s(r){var b,v;return r===e.trueLabel||r===!0?(b=e.trueLabel)!=null?b:!0:(v=e.falseLabel)!=null?v:!1}function f(r,b){m("change",s(r),b)}function k(r){if(l.value)return;const b=r.target;m("change",s(b.checked),r)}async function S(r){l.value||!a.value&&!t.value&&d.value&&(r.composedPath().some(h=>h.tagName==="LABEL")||(n.value=s([!1,e.falseLabel].includes(n.value)),await Z(),f(n.value,r)))}const u=g(()=>(c==null?void 0:c.validateEvent)||e.validateEvent);return Y(()=>e.modelValue,()=>{u.value&&(o==null||o.validate("change").catch(r=>ue()))}),{handleChange:k,onClickRoot:S}},ln=e=>{const n=z(!1),{emit:l}=X(),a=w(B,void 0),t=g(()=>T(a)===!1),d=z(!1);return{model:g({get(){var o,m;return t.value?(o=a==null?void 0:a.modelValue)==null?void 0:o.value:(m=e.modelValue)!=null?m:n.value},set(o){var m,s;t.value&&I(o)?(d.value=((m=a==null?void 0:a.max)==null?void 0:m.value)!==void 0&&o.length>(a==null?void 0:a.max.value),d.value===!1&&((s=a==null?void 0:a.changeEvent)==null||s.call(a,o))):(l(V,o),n.value=o)}}),isGroup:t,isLimitExceeded:d}},tn=(e,n,{model:l})=>{const a=w(B,void 0),t=z(!1),d=g(()=>{const s=l.value;return N(s)?s:I(s)?ke(e.label)?s.map(K).some(f=>Ne(f,e.label)):s.map(K).includes(e.label):s!=null?s===e.trueLabel:!!s}),c=W(g(()=>{var s;return(s=a==null?void 0:a.size)==null?void 0:s.value}),{prop:!0}),o=W(g(()=>{var s;return(s=a==null?void 0:a.size)==null?void 0:s.value})),m=g(()=>!!(n.default||e.label));return{checkboxButtonSize:c,isChecked:d,isFocused:t,checkboxSize:o,hasOwnLabel:m}},on=(e,{model:n})=>{function l(){I(n.value)&&!n.value.includes(e.label)?n.value.push(e.label):n.value=e.trueLabel||!0}e.checked&&l()},me=(e,n)=>{const{formItem:l}=R(),{model:a,isGroup:t,isLimitExceeded:d}=ln(e),{isFocused:c,isChecked:o,checkboxButtonSize:m,checkboxSize:s,hasOwnLabel:f}=tn(e,n,{model:a}),{isDisabled:k}=nn({model:a,isChecked:o}),{inputId:S,isLabeledByFormItem:u}=re(e,{formItemContext:l,disableIdGeneration:f,disableIdManagement:t}),{handleChange:r,onClickRoot:b}=an(e,{model:a,isLimitExceeded:d,hasOwnLabel:f,isDisabled:k,isLabeledByFormItem:u});return on(e,{model:a}),{inputId:S,isLabeledByFormItem:u,isChecked:o,isDisabled:k,isFocused:c,checkboxButtonSize:m,checkboxSize:s,hasOwnLabel:f,model:a,handleChange:r,onClickRoot:b}},sn=["tabindex","role","aria-checked"],un=["id","aria-hidden","name","tabindex","disabled","true-value","false-value"],rn=["id","aria-hidden","disabled","value","name","tabindex"],dn=L({name:"ElCheckbox"}),cn=L({...dn,props:be,emits:ve,setup(e){const n=e,l=_(),{inputId:a,isLabeledByFormItem:t,isChecked:d,isDisabled:c,isFocused:o,checkboxSize:m,hasOwnLabel:s,model:f,handleChange:k,onClickRoot:S}=me(n,l),u=D("checkbox"),r=g(()=>[u.b(),u.m(m.value),u.is("disabled",c.value),u.is("bordered",n.border),u.is("checked",d.value)]),b=g(()=>[u.e("input"),u.is("disabled",c.value),u.is("checked",d.value),u.is("indeterminate",n.indeterminate),u.is("focus",o.value)]);return(v,h)=>(C(),j(le(!i(s)&&i(t)?"span":"label"),{class:x(i(r)),"aria-controls":v.indeterminate?v.controls:null,onClick:i(S)},{default:ee(()=>[A("span",{class:x(i(b)),tabindex:v.indeterminate?0:void 0,role:v.indeterminate?"checkbox":void 0,"aria-checked":v.indeterminate?"mixed":void 0},[v.trueLabel||v.falseLabel?E((C(),y("input",{key:0,id:i(a),"onUpdate:modelValue":h[0]||(h[0]=p=>$(f)?f.value=p:null),class:x(i(u).e("original")),type:"checkbox","aria-hidden":v.indeterminate?"true":"false",name:v.name,tabindex:v.tabindex,disabled:i(c),"true-value":v.trueLabel,"false-value":v.falseLabel,onChange:h[1]||(h[1]=(...p)=>i(k)&&i(k)(...p)),onFocus:h[2]||(h[2]=p=>o.value=!0),onBlur:h[3]||(h[3]=p=>o.value=!1)},null,42,un)),[[F,i(f)]]):E((C(),y("input",{key:1,id:i(a),"onUpdate:modelValue":h[4]||(h[4]=p=>$(f)?f.value=p:null),class:x(i(u).e("original")),type:"checkbox","aria-hidden":v.indeterminate?"true":"false",disabled:i(c),value:v.label,name:v.name,tabindex:v.tabindex,onChange:h[5]||(h[5]=(...p)=>i(k)&&i(k)(...p)),onFocus:h[6]||(h[6]=p=>o.value=!0),onBlur:h[7]||(h[7]=p=>o.value=!1)},null,42,rn)),[[F,i(f)]]),A("span",{class:x(i(u).e("inner"))},null,2)],10,sn),i(s)?(C(),y("span",{key:0,class:x(i(u).e("label"))},[P(v.$slots,"default"),v.$slots.default?G("v-if",!0):(C(),y(ge,{key:0},[ne(ae(v.label),1)],64))],2)):G("v-if",!0)]),_:3},8,["class","aria-controls","onClick"]))}});var bn=O(cn,[["__file","/home/runner/work/element-plus/element-plus/packages/components/checkbox/src/checkbox.vue"]]);const vn=["name","tabindex","disabled","true-value","false-value"],mn=["name","tabindex","disabled","value"],fn=L({name:"ElCheckboxButton"}),hn=L({...fn,props:be,emits:ve,setup(e){const n=e,l=_(),{isFocused:a,isChecked:t,isDisabled:d,checkboxButtonSize:c,model:o,handleChange:m}=me(n,l),s=w(B,void 0),f=D("checkbox"),k=g(()=>{var u,r,b,v;const h=(r=(u=s==null?void 0:s.fill)==null?void 0:u.value)!=null?r:"";return{backgroundColor:h,borderColor:h,color:(v=(b=s==null?void 0:s.textColor)==null?void 0:b.value)!=null?v:"",boxShadow:h?`-1px 0 0 0 ${h}`:void 0}}),S=g(()=>[f.b("button"),f.bm("button",c.value),f.is("disabled",d.value),f.is("checked",t.value),f.is("focus",a.value)]);return(u,r)=>(C(),y("label",{class:x(i(S))},[u.trueLabel||u.falseLabel?E((C(),y("input",{key:0,"onUpdate:modelValue":r[0]||(r[0]=b=>$(o)?o.value=b:null),class:x(i(f).be("button","original")),type:"checkbox",name:u.name,tabindex:u.tabindex,disabled:i(d),"true-value":u.trueLabel,"false-value":u.falseLabel,onChange:r[1]||(r[1]=(...b)=>i(m)&&i(m)(...b)),onFocus:r[2]||(r[2]=b=>a.value=!0),onBlur:r[3]||(r[3]=b=>a.value=!1)},null,42,vn)),[[F,i(o)]]):E((C(),y("input",{key:1,"onUpdate:modelValue":r[4]||(r[4]=b=>$(o)?o.value=b:null),class:x(i(f).be("button","original")),type:"checkbox",name:u.name,tabindex:u.tabindex,disabled:i(d),value:u.label,onChange:r[5]||(r[5]=(...b)=>i(m)&&i(m)(...b)),onFocus:r[6]||(r[6]=b=>a.value=!0),onBlur:r[7]||(r[7]=b=>a.value=!1)},null,42,mn)),[[F,i(o)]]),u.$slots.default||u.label?(C(),y("span",{key:2,class:x(i(f).be("button","inner")),style:pe(i(t)?i(k):void 0)},[P(u.$slots,"default",{},()=>[ne(ae(u.label),1)])],6)):G("v-if",!0)],2))}});var fe=O(hn,[["__file","/home/runner/work/element-plus/element-plus/packages/components/checkbox/src/checkbox-button.vue"]]);const kn=xe({modelValue:{type:Ce(Array),default:()=>[]},disabled:Boolean,min:Number,max:Number,size:ie,label:String,fill:String,textColor:String,tag:{type:String,default:"div"},validateEvent:{type:Boolean,default:!0}}),gn={[V]:e=>I(e),change:e=>I(e)},pn=L({name:"ElCheckboxGroup"}),xn=L({...pn,props:kn,emits:gn,setup(e,{emit:n}){const l=e,a=D("checkbox"),{formItem:t}=R(),{inputId:d,isLabeledByFormItem:c}=re(l,{formItemContext:t}),o=async s=>{n(V,s),await Z(),n("change",s)},m=g({get(){return l.modelValue},set(s){o(s)}});return ye(B,{...en(Se(l),["size","min","max","disabled","validateEvent","fill","textColor"]),modelValue:m,changeEvent:o}),Y(()=>l.modelValue,()=>{l.validateEvent&&(t==null||t.validate("change").catch(s=>ue()))}),(s,f)=>{var k;return C(),j(le(s.tag),{id:i(d),class:x(i(a).b("group")),role:"group","aria-label":i(c)?void 0:s.label||"checkbox-group","aria-labelledby":i(c)?(k=i(t))==null?void 0:k.labelId:void 0},{default:ee(()=>[P(s.$slots,"default")]),_:3},8,["id","class","aria-label","aria-labelledby"])}}});var he=O(xn,[["__file","/home/runner/work/element-plus/element-plus/packages/components/checkbox/src/checkbox-group.vue"]]);const Ln=Le(bn,{CheckboxButton:fe,CheckboxGroup:he});te(fe);te(he);export{Ln as E,ce as b};

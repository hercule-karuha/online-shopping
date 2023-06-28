import{a8 as se,H as Q,J as O,K,d as E,o as f,c as F,i as A,U as w,j as k,a as e,t as W,l as T,m as y,C as M,a9 as ae,E as ne,q as P,n as X,N as x,I as ie,aF as re,g as ue,r as I,aG as de,B as J,M as ce,e as fe,aH as pe,aI as G,aq as ye,Y as me,L as ge,S as ve,w as Ce,aj as be,ai as he,a1 as ke,au as we,aJ as De,a2 as Be,_ as Ie,x as H,F as Ee,p as Fe}from"./index-43945a83.js";import{s as $e,g as Se,F as _e,B as Ae,D as Te,U as ee,G as Me,H as Y,J as Pe,K as Ne,d as Z,L as Re,M as Le,N as Oe,A as ze}from"./request-d304319d.js";const Ve=(...o)=>i=>{o.forEach(a=>{se(a)?a(i):a.value=i})},oe=Symbol("dialogInjectionKey"),le=Q({center:{type:Boolean,default:!1},alignCenter:{type:Boolean,default:!1},closeIcon:{type:$e},customClass:{type:String,default:""},draggable:{type:Boolean,default:!1},fullscreen:{type:Boolean,default:!1},showClose:{type:Boolean,default:!0},title:{type:String,default:""}}),Ue={close:()=>!0},qe=["aria-label"],je=["id"],Ke=O({name:"ElDialogContent"}),Je=O({...Ke,props:le,emits:Ue,setup(o){const i=o,{t:a}=Se(),{Close:r}=Te,{dialogRef:d,headerRef:m,bodyId:C,ns:s,style:n}=K(oe),{focusTrapRef:p}=K(_e),g=Ve(p,d),b=E(()=>i.draggable);return Ae(d,m,b),(t,c)=>(f(),F("div",{ref:e(g),class:k([e(s).b(),e(s).is("fullscreen",t.fullscreen),e(s).is("draggable",e(b)),e(s).is("align-center",t.alignCenter),{[e(s).m("center")]:t.center},t.customClass]),style:X(e(n)),tabindex:"-1"},[A("header",{ref_key:"headerRef",ref:m,class:k(e(s).e("header"))},[w(t.$slots,"header",{},()=>[A("span",{role:"heading",class:k(e(s).e("title"))},W(t.title),3)]),t.showClose?(f(),F("button",{key:0,"aria-label":e(a)("el.dialog.close"),class:k(e(s).e("headerbtn")),type:"button",onClick:c[0]||(c[0]=N=>t.$emit("close"))},[T(e(ne),{class:k(e(s).e("close"))},{default:y(()=>[(f(),M(ae(t.closeIcon||e(r))))]),_:1},8,["class"])],10,qe)):P("v-if",!0)],2),A("div",{id:e(C),class:k(e(s).e("body"))},[w(t.$slots,"default")],10,je),t.$slots.footer?(f(),F("footer",{key:0,class:k(e(s).e("footer"))},[w(t.$slots,"footer")],2)):P("v-if",!0)],6))}});var Ge=x(Je,[["__file","/home/runner/work/element-plus/element-plus/packages/components/dialog/src/dialog-content.vue"]]);const He=Q({...le,appendToBody:{type:Boolean,default:!1},beforeClose:{type:ie(Function)},destroyOnClose:{type:Boolean,default:!1},closeOnClickModal:{type:Boolean,default:!0},closeOnPressEscape:{type:Boolean,default:!0},lockScroll:{type:Boolean,default:!0},modal:{type:Boolean,default:!0},openDelay:{type:Number,default:0},closeDelay:{type:Number,default:0},top:{type:String},modelValue:{type:Boolean,default:!1},modalClass:String,width:{type:[String,Number]},zIndex:{type:Number},trapFocus:{type:Boolean,default:!1}}),Ye={open:()=>!0,opened:()=>!0,close:()=>!0,closed:()=>!0,[ee]:o=>re(o),openAutoFocus:()=>!0,closeAutoFocus:()=>!0},Ze=(o,i)=>{const r=ue().emit,{nextZIndex:d}=Me();let m="";const C=Y(),s=Y(),n=I(!1),p=I(!1),g=I(!1),b=I(o.zIndex||d());let t,c;const N=Pe("namespace",pe),z=E(()=>{const u={},B=`--${N.value}-dialog`;return o.fullscreen||(o.top&&(u[`${B}-margin-top`]=o.top),o.width&&(u[`${B}-width`]=de(o.width))),u}),V=E(()=>o.alignCenter?{display:"flex"}:{});function U(){r("opened")}function R(){r("closed"),r(ee,!1),o.destroyOnClose&&(g.value=!1)}function q(){r("close")}function L(){c==null||c(),t==null||t(),o.openDelay&&o.openDelay>0?{stop:t}=G(()=>v(),o.openDelay):v()}function $(){t==null||t(),c==null||c(),o.closeDelay&&o.closeDelay>0?{stop:c}=G(()=>_(),o.closeDelay):_()}function S(){function u(B){B||(p.value=!0,n.value=!1)}o.beforeClose?o.beforeClose(u):$()}function j(){o.closeOnClickModal&&S()}function v(){ye&&(n.value=!0)}function _(){n.value=!1}function l(){r("openAutoFocus")}function h(){r("closeAutoFocus")}function D(u){var B;((B=u.detail)==null?void 0:B.focusReason)==="pointer"&&u.preventDefault()}o.lockScroll&&Ne(n);function te(){o.closeOnPressEscape&&S()}return J(()=>o.modelValue,u=>{u?(p.value=!1,L(),g.value=!0,b.value=o.zIndex?b.value++:d(),ce(()=>{r("open"),i.value&&(i.value.scrollTop=0)})):n.value&&$()}),J(()=>o.fullscreen,u=>{i.value&&(u?(m=i.value.style.transform,i.value.style.transform=""):i.value.style.transform=m)}),fe(()=>{o.modelValue&&(n.value=!0,g.value=!0,L())}),{afterEnter:U,afterLeave:R,beforeLeave:q,handleClose:S,onModalClick:j,close:$,doClose:_,onOpenAutoFocus:l,onCloseAutoFocus:h,onCloseRequested:te,onFocusoutPrevented:D,titleId:C,bodyId:s,closed:p,style:z,overlayDialogStyle:V,rendered:g,visible:n,zIndex:b}},Qe=["aria-label","aria-labelledby","aria-describedby"],We=O({name:"ElDialog",inheritAttrs:!1}),Xe=O({...We,props:He,emits:Ye,setup(o,{expose:i}){const a=o,r=me();Z({scope:"el-dialog",from:"the title slot",replacement:"the header slot",version:"3.0.0",ref:"https://element-plus.org/en-US/component/dialog.html#slots"},E(()=>!!r.title)),Z({scope:"el-dialog",from:"custom-class",replacement:"class",version:"2.3.0",ref:"https://element-plus.org/en-US/component/dialog.html#attributes",type:"Attribute"},E(()=>!!a.customClass));const d=ge("dialog"),m=I(),C=I(),s=I(),{visible:n,titleId:p,bodyId:g,style:b,overlayDialogStyle:t,rendered:c,zIndex:N,afterEnter:z,afterLeave:V,beforeLeave:U,handleClose:R,onModalClick:q,onOpenAutoFocus:L,onCloseAutoFocus:$,onCloseRequested:S,onFocusoutPrevented:j}=Ze(a,m);ve(oe,{dialogRef:m,headerRef:C,bodyId:g,ns:d,rendered:c,style:b});const v=Oe(q),_=E(()=>a.draggable&&!a.fullscreen);return i({visible:n,dialogContentRef:s}),(l,h)=>(f(),M(De,{to:"body",disabled:!l.appendToBody},[T(we,{name:"dialog-fade",onAfterEnter:e(z),onAfterLeave:e(V),onBeforeLeave:e(U),persisted:""},{default:y(()=>[Ce(T(e(Re),{"custom-mask-event":"",mask:l.modal,"overlay-class":l.modalClass,"z-index":e(N)},{default:y(()=>[A("div",{role:"dialog","aria-modal":"true","aria-label":l.title||void 0,"aria-labelledby":l.title?void 0:e(p),"aria-describedby":e(g),class:k(`${e(d).namespace.value}-overlay-dialog`),style:X(e(t)),onClick:h[0]||(h[0]=(...D)=>e(v).onClick&&e(v).onClick(...D)),onMousedown:h[1]||(h[1]=(...D)=>e(v).onMousedown&&e(v).onMousedown(...D)),onMouseup:h[2]||(h[2]=(...D)=>e(v).onMouseup&&e(v).onMouseup(...D))},[T(e(Le),{loop:"",trapped:e(n),"focus-start-el":"container",onFocusAfterTrapped:e(L),onFocusAfterReleased:e($),onFocusoutPrevented:e(j),onReleaseRequested:e(S)},{default:y(()=>[e(c)?(f(),M(Ge,be({key:0,ref_key:"dialogContentRef",ref:s},l.$attrs,{"custom-class":l.customClass,center:l.center,"align-center":l.alignCenter,"close-icon":l.closeIcon,draggable:e(_),fullscreen:l.fullscreen,"show-close":l.showClose,title:l.title,onClose:e(R)}),he({header:y(()=>[l.$slots.title?w(l.$slots,"title",{key:1}):w(l.$slots,"header",{key:0,close:e(R),titleId:e(p),titleClass:e(d).e("title")})]),default:y(()=>[w(l.$slots,"default")]),_:2},[l.$slots.footer?{name:"footer",fn:y(()=>[w(l.$slots,"footer")])}:void 0]),1040,["custom-class","center","align-center","close-icon","draggable","fullscreen","show-close","title","onClose"])):P("v-if",!0)]),_:3},8,["trapped","onFocusAfterTrapped","onFocusAfterReleased","onFocusoutPrevented","onReleaseRequested"])],46,Qe)]),_:3},8,["mask","overlay-class","z-index"]),[[ke,e(n)]])]),_:3},8,["onAfterEnter","onAfterLeave","onBeforeLeave"])],8,["disabled"]))}});var xe=x(Xe,[["__file","/home/runner/work/element-plus/element-plus/packages/components/dialog/src/dialog.vue"]]);const eo=Be(xe);const oo={class:"dialog-body"},lo={key:0,class:"dialog-footer"},to={__name:"DialogModule",props:{show:{type:Boolean,default:!0},title:{type:String,default:""},showClose:{type:Boolean,default:!0},width:{type:String,default:"30%"},top:{type:String,default:"100px"},buttons:{type:Array},showCancel:{type:Boolean,default:!0}},emits:["close"],setup(o,{emit:i}){const a=o,r=()=>{i("close")};return(d,m)=>{const C=ze,s=eo;return f(),F("div",null,[T(s,{class:"dialog-container","model-value":a.show,"show-close":a.showClose,"close-on-click-modal":!1,title:a.title,width:a.width,"lock-scroll":!1,top:a.top,onClose:r},{default:y(()=>[A("div",oo,[w(d.$slots,"default",{},void 0,!0)]),o.buttons&&o.buttons.length>0||o.showCancel?(f(),F("div",lo,[o.showCancel?(f(),M(C,{key:0,type:"warning",onClick:r},{default:y(()=>[H("取消")]),_:1})):P("",!0),(f(!0),F(Ee,null,Fe(o.buttons,(n,p)=>(f(),M(C,{key:p,type:n.type,onClick:n.click},{default:y(()=>[H(W(n.text),1)]),_:2},1032,["type","onClick"]))),128))])):P("",!0)]),_:3},8,["model-value","show-close","title","width","top"])])}}},no=Ie(to,[["__scopeId","data-v-6097bcf7"]]);export{no as D};
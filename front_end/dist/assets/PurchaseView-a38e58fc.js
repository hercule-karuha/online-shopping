import{aC as Y,a4 as Z,aD as ee,aE as se,H as ae,J,L as te,d as le,o,c as i,i as s,U as D,a as r,C as S,j as k,a9 as oe,q as w,t as I,N as ne,a2 as re,_ as ue,r as v,u as ie,e as de,B as ce,f as pe,$ as me,m as u,l as n,F as L,p as q,x as C,y as ve,z as fe}from"./index-43945a83.js";import{a as _e,b as he,E as ge}from"./el-form-item-a39e9bf3.js";import{E as be}from"./el-checkbox-4dbd9916.js";import"./el-input-853da7da.js";import"./el-popper-a281cd70.js";import{E as ye,a as ke}from"./el-radio-8b78f2fb.js";import{D as we}from"./DialogModule-26653baa.js";/* empty css                       */import{u as Ve,i as Ce}from"./purchaseList-c3137989.js";import{p as M}from"./tools-e8160517.js";import{P as Ie}from"./PurchaseListItem-07a4cb34.js";import{m as T,A as Ee,e as Ae}from"./request-d304319d.js";import{s as De}from"./element-china-area-data-7e4514d6.js";import{getDetailUserInfo as Se,editUserInfo as $e}from"./user-f47ffe26.js";const V={success:"icon-success",warning:"icon-warning",error:"icon-error",info:"icon-info"},j={[V.success]:Y,[V.warning]:Z,[V.error]:ee,[V.info]:se},Ne=ae({title:{type:String,default:""},subTitle:{type:String,default:""},icon:{type:String,values:["success","warning","info","error"],default:"info"}}),Pe=J({name:"ElResult"}),Ue=J({...Pe,props:Ne,setup(h){const d=h,f=te("result"),g=le(()=>{const e=d.icon,p=e&&V[e]?V[e]:"icon-info",b=j[p]||j["icon-info"];return{class:p,component:b}});return(e,p)=>(o(),i("div",{class:k(r(f).b())},[s("div",{class:k(r(f).e("icon"))},[D(e.$slots,"icon",{},()=>[r(g).component?(o(),S(oe(r(g).component),{key:0,class:k(r(g).class)},null,8,["class"])):w("v-if",!0)])],2),e.title||e.$slots.title?(o(),i("div",{key:0,class:k(r(f).e("title"))},[D(e.$slots,"title",{},()=>[s("p",null,I(e.title),1)])],2)):w("v-if",!0),e.subTitle||e.$slots["sub-title"]?(o(),i("div",{key:1,class:k(r(f).e("subtitle"))},[D(e.$slots,"sub-title",{},()=>[s("p",null,I(e.subTitle),1)])],2)):w("v-if",!0),e.$slots.extra?(o(),i("div",{key:2,class:k(r(f).e("extra"))},[D(e.$slots,"extra")],2)):w("v-if",!0)],2))}});var Re=ne(Ue,[["__file","/home/runner/work/element-plus/element-plus/packages/components/result/src/result.vue"]]);const xe=re(Re);const $=h=>(ve("data-v-74c21543"),h=h(),fe(),h),Be=$(()=>s("header",null,"下单",-1)),Fe={key:0,class:"content"},Le={class:"address"},qe=$(()=>s("div",{class:"label"},[s("span",null,"请选你的收货地址")],-1)),Me={class:"add"},Te={class:"detail"},je=$(()=>s("div",{class:"label"},[s("span",null,"确认订单信息")],-1)),Je=$(()=>s("div",{class:"info"},[s("div",{class:"left"},[s("span",null,"商品")]),s("span",null,"单价"),s("span",null,"数量"),s("span",null,"小计")],-1)),ze={class:"list"},Oe={class:"all"},Ge={class:"all-wrap"},He={class:"all-info"},Ke={class:"total-price"},Qe={class:"address-info"},We={key:0},Xe={class:"op"},Ye={class:"dialog-body"},Ze={__name:"PurchaseView",setup(h){const d=Ve(),f=De,g=v(null),e=v({}),p=v(0),b=ie();let E=null;const R=v(!1),N=v(5);d.list.forEach(l=>{p.value+=l.num*l.price});const y=v([]),c=v({});de(async()=>{if(d.list.length===0){b.replace("/user/shoppingCart");return}const l=await Se();if(!l||(c.value=l.data,!l.data.address))return;c.value.address=JSON.parse(l.data.address);let a=c.value.address.areaArr.join("  ");y.value.push({address:a,phone:l.data.phone}),console.log(y.value)}),ce(d.list,l=>{p.value=0,l.forEach(a=>{p.value+=a.num*a.price})},{immediate:!0}),pe(()=>{d.list=[],clearInterval(E)});const m=v(y.value[0]),_=me({show:!1,title:"添加新地址",buttons:[{text:"取消",type:"default",click:()=>{_.show=!1}},{text:"确认",type:"primary",click:()=>{g.value.validate(l=>{if(l){if(y.value.push({address:M(e.value.area).join(" ")+" "+e.value.detailAddress+"  ",phone:e.value.phone}),_.show=!1,e.value.setDefault){c.value.address={codeArr:e.value.area,areaArr:M(e.value.area),detailAddress:e.value.detailAddress},c.value.phone=e.value.phone;const a=new FormData;a.append("address",JSON.stringify(c.value.address)),a.append("phone",c.value.phone),a.append("userName",c.value.userName),a.append("gender",c.value.sex),$e(a)}}else return!1})}}]}),z=()=>{_.show=!0},O={phone:[{required:!0,message:"请输入手机号",trigger:"blur"},{pattern:/^1[3456789]\d{9}$/,message:"手机号格式不正确",trigger:"blur"}],area:[{required:!0,message:"请选择所在地区",trigger:"blur"}],detailAddress:[{required:!0,message:"请输入详细地址",trigger:"blur"}]},G=async()=>{if(!m.value){T.warning("请选择一个收货地址");return}const l=d.list.map(A=>({productId:A.productId,num:A.num.toString()})),a={address:m.value.address,phone:m.value.phone,list:l};await Ce(a)&&(d.list=[],T.success("提交订单成功"),R.value=!0,E=setInterval(()=>{N.value--,N.value===0&&(clearInterval(E),b.replace("/user"))},1e3))},H=()=>{clearInterval(E),b.replace("/user")};return(l,a)=>{const x=ke,A=ye,B=Ee,K=xe,F=Ae,P=_e,Q=he,W=be,X=ge;return o(),i("main",null,[Be,R.value?(o(),S(K,{key:1,icon:"success",title:"购买成功","sub-title":N.value+"秒后返回购物车"},{extra:u(()=>[n(B,{type:"primary",onClick:H},{default:u(()=>[C("立即返回")]),_:1})]),_:1},8,["sub-title"])):(o(),i("div",Fe,[s("div",Le,[qe,y.value.length>0?(o(),S(A,{key:0,modelValue:m.value,"onUpdate:modelValue":a[0]||(a[0]=t=>m.value=t)},{default:u(()=>[(o(!0),i(L,null,q(y.value,(t,U)=>(o(),i("div",{key:U,class:"add-item"},[n(x,{default:"",label:t,"model-value":t},{default:u(()=>[C(I(t.address+" "+t.phone),1)]),_:2},1032,["label","model-value"])]))),128))]),_:1},8,["modelValue"])):w("",!0),s("div",Me,[n(B,{type:"primary",onClick:z},{default:u(()=>[C("使用 新地址")]),_:1})])]),s("div",Te,[je,Je,s("div",ze,[(o(!0),i(L,null,q(r(d).list,(t,U)=>(o(),S(Ie,{class:"list-item",key:U,data:t},null,8,["data"]))),128))]),s("div",Oe,[s("div",Ge,[s("div",He,[s("div",Ke,[C("实付款¥"),s("span",null,I(p.value.toFixed(2)),1)]),s("div",Qe,[C("寄送至: "),m.value?(o(),i("span",We,I(m.value.address+"  "+m.value.phone),1)):w("",!0)])]),s("div",Xe,[s("div",{class:"back",onClick:a[1]||(a[1]=t=>r(b).go(-1))},"返回购物车"),s("div",{class:"submit",onClick:G},"提交订单")])])])])])),n(we,{show:_.show,title:_.title,buttons:_.buttons,showCancel:!1,onClose:a[6]||(a[6]=t=>_.show=!1)},{default:u(()=>[s("div",Ye,[n(X,{ref_key:"formDataRef",ref:g,model:e.value,rules:O,"label-position":"top"},{default:u(()=>[n(P,{label:"手机号",prop:"phone"},{default:u(()=>[n(F,{modelValue:e.value.phone,"onUpdate:modelValue":a[2]||(a[2]=t=>e.value.phone=t)},null,8,["modelValue"])]),_:1}),n(P,{label:"所在地区",prop:"area"},{default:u(()=>[n(Q,{options:r(f),modelValue:e.value.area,"onUpdate:modelValue":a[3]||(a[3]=t=>e.value.area=t)},null,8,["options","modelValue"])]),_:1}),n(P,{label:"详细地址",prop:"detailAddress"},{default:u(()=>[n(F,{modelValue:e.value.detailAddress,"onUpdate:modelValue":a[4]||(a[4]=t=>e.value.detailAddress=t)},null,8,["modelValue"])]),_:1}),n(W,{modelValue:e.value.setDefault,"onUpdate:modelValue":a[5]||(a[5]=t=>e.value.setDefault=t),label:"设置为默认地址"},null,8,["modelValue"])]),_:1},8,["model"])])]),_:1},8,["show","title","buttons"])])}}},vs=ue(Ze,[["__scopeId","data-v-74c21543"]]);export{vs as default};

import{_ as b,g as F,u as z,o as a,c as l,a as d,n as H,b as q,r as g,d as P,e as J,f as K,h as k,i as e,j,w as G,v as Q,k as W,l as n,m as _,F as X,p as Y,q as C,t as $,E as Z,s as L,x as ee,y as se,z as oe}from"./index-43945a83.js";import{D as te}from"./DataList-e4682188.js";import{m as ne}from"./request-d304319d.js";import{P as ae}from"./ProductItem-fd5e135e.js";import{g as le}from"./product-252d41f2.js";import"./el-input-853da7da.js";import"./el-popper-a281cd70.js";const re=["src"],ue={key:1,class:"fake-avatar"},ce={__name:"AvatarModule",props:{userId:{type:Number,default:""},size:{type:Number,default:50},addLink:{type:Boolean,default:!0}},setup(r){const c=r,{proxy:p}=F(),i=z(),f=()=>{c.addLink&&i.push({path:"/user",query:{userId:c.userId}})};return(u,v)=>(a(),l("div",{class:"content",style:H({width:r.size+"px",height:r.size+"px"})},[r.userId!==""?(a(),l("img",{key:0,src:d(p).globalInfo.avatarUrl+r.userId,onClick:f},null,8,re)):(a(),l("div",ue," 未登录 "))],4))}},ie=b(ce,[["__scopeId","data-v-6a16620a"]]);const m=r=>(se("data-v-2a485bcf"),r=r(),oe(),r),de=m(()=>e("div",{class:"logo"},[e("span",null,"L"),e("span",null,"O"),e("span",null,"G"),e("span",null,"O")],-1)),pe={class:"search-box"},ve={key:0,class:"record"},_e={key:0},me={key:1},fe={class:"record-list"},he=["onClick"],ge=["onClick"],ke=m(()=>e("div",{class:"recommend"},null,-1)),ye={class:"user-info"},Ce={class:"avatar"},Ie={class:"info"},Se={class:"login"},we={class:"func"},xe=m(()=>e("span",null,"购物车",-1)),Ne=m(()=>e("span",null,"购买记录",-1)),$e=m(()=>e("span",null,"我的店铺",-1)),Le={class:"homepage-list"},be=m(()=>e("span",null,"猜你喜欢",-1)),ze={__name:"HomeView",setup(r){const c=q(),p=g(""),i=P(()=>c.userInfo,{immediate:!0}),f=g(),u=z(),v=g(localStorage.getItem("searchLogs")?JSON.parse(localStorage.getItem("searchLogs")):[]),w=g({list:[]}),y=g(!1),V=s=>{y.value=s},B=s=>{v.value.splice(s,1),console.log("del")},I=s=>{if(console.log(s),!!s){if(s.trim()==""){ne.waning("请输入关键词");return}v.value.unshift(s),localStorage.setItem("searchLogs",JSON.stringify(v.value)),u.push({path:"/product/list",query:{keyword:s}})}};J(async()=>{window.addEventListener("click",x);const s=await le({pageNo:"1",pageSize:"15"});s&&(w.value=s.data)}),K(()=>{window.removeEventListener("click",x)});const x=s=>{f.value&&f.value.contains(s.target)||(y.value=!1)},N=s=>{u.push("/account/"+s)},R=s=>{console.log(s)},M=()=>{c.userInfo.userId?u.push("/store/newStore"):u.push("/account/login")};return(s,o)=>{const E=k("Search"),h=Z,O=k("Close"),T=k("ShoppingCart"),U=k("Clock"),A=k("TakeawayBox");return a(),l("main",null,[e("header",null,[de,e("div",pe,[e("div",{ref_key:"searchRef",ref:f,class:j(["search",y.value===!0?"search-record":""])},[G(e("input",{type:"text","onUpdate:modelValue":o[0]||(o[0]=t=>p.value=t),onKeyup:o[1]||(o[1]=W(t=>I(p.value),["enter"])),placeholder:"请输入你要搜索的关键词",onFocus:o[2]||(o[2]=t=>V(!0))},null,544),[[Q,p.value]]),e("div",{class:"btn",onClick:o[3]||(o[3]=t=>I(p.value))},[n(h,null,{default:_(()=>[n(E)]),_:1})]),y.value?(a(),l("div",ve,[v.value.length>0?(a(),l("p",_e," 搜索历史 ")):(a(),l("p",me," 暂无历史记录 ")),e("div",fe,[(a(!0),l(X,null,Y(v.value,(t,S)=>(a(),l("div",{key:S,class:"record-item",onClick:L(D=>I(t),["stop"])},[ee($(t)+" ",1),e("span",{class:"del-record",onClick:L(D=>B(S),["stop"])},[n(h,null,{default:_(()=>[n(O)]),_:1})],8,ge)],8,he))),128))])])):C("",!0)],2)])]),e("nav",null,[ke,e("div",ye,[e("div",Ce,[n(ie,{"user-id":d(c).userInfo?d(c).userInfo.userId:"",size:70},null,8,["user-id"])]),e("div",Ie,[e("p",null,"Hi! "+$(i.value.userName?i.value.userName:"你好"),1),e("div",Se,[i.value.userName?C("",!0):(a(),l("div",{key:0,onClick:o[4]||(o[4]=t=>N("login"))}," 登录 ")),i.value.userName?C("",!0):(a(),l("div",{key:1,onClick:o[5]||(o[5]=t=>N("register"))}," 注册 ")),i.value.userType==0?(a(),l("div",{key:2,onClick:M}," 开店 ")):C("",!0)]),e("div",we,[e("div",{onClick:o[6]||(o[6]=t=>d(u).push("/user/shoppingCart"))},[n(h,null,{default:_(()=>[n(T)]),_:1}),xe]),e("div",{onClick:o[7]||(o[7]=t=>d(u).push("/user/order"))},[n(h,null,{default:_(()=>[n(U)]),_:1}),Ne]),e("div",{onClick:o[8]||(o[8]=t=>d(u).push("/user/stores"))},[n(h,null,{default:_(()=>[n(A)]),_:1}),$e])])])])]),e("div",Le,[be,n(te,{"data-source":w.value,onPageNoChange:R},{default:_(({data:t})=>[n(ae,{data:t,onClick:S=>d(u).push("/product/detail/"+t.productId)},null,8,["data","onClick"])]),_:1},8,["data-source"])])])}}},Ue=b(ze,[["__scopeId","data-v-2a485bcf"]]);export{Ue as default};

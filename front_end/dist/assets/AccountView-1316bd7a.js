import{_ as z,A,u as T,b as R,B as q,e as D,f as L,r as c,h as j,o as n,c as I,i as o,j as b,t as p,w as _,v as y,C as V,m as u,q as v,E as M,l as m,x as k,y as X,z as $}from"./index-43945a83.js";/* empty css                       */import{E as G,a as O}from"./el-radio-8b78f2fb.js";import{m as l}from"./request-d304319d.js";import{login as F,register as H}from"./user-f47ffe26.js";const U=i=>(X("data-v-392035d8"),i=i(),$(),i),J={class:"content"},K={class:"title"},Q={class:"account"},W={class:"password"},Y={key:0,class:"password"},Z=U(()=>o("p",null,[k("欢迎来到"),o("span",null,"XX商城")],-1)),ee=U(()=>o("div",{class:"img"},[o("img",{src:"https://pic.imgdb.cn/item/6488985b1ddac507cc1f59cd.jpg?w=300",alt:""})],-1)),se={__name:"AccountView",setup(i){const h=A(),d=T(),f=R();q(()=>h.params.type,t=>{a.value=t}),D(()=>{f.needLogin&&(R.needLogin=!1,l.warning("请先登录"))}),L(()=>{f.needLogin=!1});const a=c(h.params.type),e=c({gender:"1"}),w=c(null),g=c(null),B=()=>{a.value==="login"?(e.value={gender:"1"},d.replace("/account/register")):d.replace("/account/login")},E=async()=>{if(!e.value.userName){l.error("请输入用户名",null);return}if(!e.value.password){l.error("请输入密码");return}if(a.value=="login"){if(e.value.password.length<8){l.error("密码长度不能小于8位");return}const t={userName:e.value.userName,password:e.value.password},s=await F(t);s&&(f.userInfo=s.data,l.success("登录成功"),d.go(-1))}else{if(e.value.password.length<8){l.error("密码长度不能小于8位");return}if(e.value.password!==e.value.confirmPassword){l.error("两次密码不一致");return}const t={userName:e.value.userName,password:e.value.password,gender:e.value.gender};await H(t)&&(l.success("注册成功, 请登录"),d.replace("/account/login"))}},x=t=>{t==="password"?w.value.type=w.value.type==="password"?"text":"password":g.value.type=g.value.type==="password"?"text":"password"};return(t,s)=>{const N=j("View"),C=M,P=O,S=G;return n(),I("main",null,[o("div",J,[o("div",{class:b(["login",a.value=="register"?"register":""])},[o("p",K,p(a.value.toUpperCase()),1),o("div",Q,[_(o("input",{"onUpdate:modelValue":s[0]||(s[0]=r=>e.value.userName=r),type:"text",placeholder:"用户名"},null,512),[[y,e.value.userName]])]),o("div",W,[_(o("input",{ref_key:"passwordRef",ref:w,"onUpdate:modelValue":s[1]||(s[1]=r=>e.value.password=r),type:"password",placeholder:"密码"},null,512),[[y,e.value.password]]),e.value.password&&e.value.password.length>0?(n(),V(C,{key:0,onClick:s[2]||(s[2]=r=>x("password"))},{default:u(()=>[m(N)]),_:1})):v("",!0)]),a.value==="register"?(n(),I("div",Y,[_(o("input",{"onUpdate:modelValue":s[3]||(s[3]=r=>e.value.confirmPassword=r),type:"password",ref_key:"confirmPasswordRef",ref:g,placeholder:"确认密码"},null,512),[[y,e.value.confirmPassword]]),e.value.confirmPassword&&e.value.confirmPassword.length>0?(n(),V(C,{key:0,onClick:s[4]||(s[4]=r=>x("confirmPassword"))},{default:u(()=>[m(N)]),_:1})):v("",!0)])):v("",!0),a.value==="register"?(n(),V(S,{key:1,modelValue:e.value.gender,"onUpdate:modelValue":s[5]||(s[5]=r=>e.value.gender=r),class:"radio-group"},{default:u(()=>[m(P,{label:"1",size:"large"},{default:u(()=>[k(" 男 ")]),_:1}),m(P,{label:"2",size:"large"},{default:u(()=>[k(" 女 ")]),_:1})]),_:1},8,["modelValue"])):v("",!0),o("button",{onClick:E},p(a.value==="login"?"登录":"注册"),1)],2),o("div",{class:b(["cover",a.value=="register"?"register-cover":""])},[Z,ee,o("span",null,p(a.value==="login"?"没有":"已有")+"账号?",1),o("button",{onClick:B}," 去"+p(a.value==="login"?"注册":"登录"),1)],2)])])}}},ne=z(se,[["__scopeId","data-v-392035d8"]]);export{ne as default};

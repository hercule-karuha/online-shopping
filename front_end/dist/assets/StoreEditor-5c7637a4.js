import{_ as y,r as c,g as w,u as E,A as x,b as h,e as C,o as D,c as N,i as v,t as q,l as r,m as i,a as F}from"./index-43945a83.js";import{E as k,a as B,b as J}from"./el-form-item-a39e9bf3.js";import"./el-input-853da7da.js";import"./el-popper-a281cd70.js";import"./el-checkbox-4dbd9916.js";import"./el-radio-8b78f2fb.js";import{_ as O}from"./CoverUpload-1632b343.js";import{s as M}from"./element-china-area-data-7e4514d6.js";import{g as j,n as z,e as G}from"./store-a6f281e4.js";import{m,e as H}from"./request-d304319d.js";import{p as K}from"./tools-e8160517.js";import{getUserInfo as A}from"./user-f47ffe26.js";const L={class:"content"},P={class:"header"},Q={__name:"StoreEditor",setup(T){const _=c(""),l=c(!0),{proxy:b}=w(),V=M,p=E(),g=x(),n=h(),e=c({selectedRegion:["35","3501","350121"]}),I=c(null),R=s=>{e.value.cover=s};C(async()=>{if(g.path==="/store/edit"&&(l.value=!1),n.userInfo==={}||!n.userInfo){const s=await A();n.userInfo=s.data}if(g.path==="/store/edit"){n.userInfo.storeId==0&&p.push("/"),l.value=!1;const s=await j(n.userInfo.storeId);if(s){const a=s.data;_.value=b.globalInfo.storeCoverUrl+n.userInfo.storeId,a.address=JSON.parse(a.address),e.value.name=a.name,e.value.storeId=a.storeId,e.value.detailAddress=a.address.detailAddress,e.value.selectedRegion=a.address.codeArr}else return}});const S=async()=>{I.value.validate(async s=>{if(s){const a=K(e.value.selectedRegion);let o={codeArr:e.value.selectedRegion,areaArr:a,detailAddress:e.value.detailAddress};o=JSON.stringify(o);let t=new FormData;if(t.append("storeName",e.value.name),t.append("cover",e.value.cover),t.append("address",o),l.value){const d=await z(t);if(!d)m.error("开店失败");else{m.success("开店成功");const f=await A();n.userInfo=f.data,p.push("/store/detail/"+d.data.storeId)}}else t.get("cover")=="undefined"&&t.delete("cover"),t.append("storeId",e.value.storeId),await G(t)?(m.success("修改店铺信息成功"),p.push("/store/detail/"+e.value.storeId)):m.error("修改店铺信息失败")}else return!1})},U={name:[{required:!0,message:"请输入商店名称",trigger:"change"},{min:2,max:20,message:"长度在 2 到 20 个字符",trigger:"change"}],selectedRegion:[{required:!0,message:"请选择发货地址"}],detailAddress:[{required:!0,message:"请输入详细地址",trigger:"change"}],cover:[{validator:(s,a,o)=>{l.value&&!e.value.cover?o(new Error("请上传商店封面")):o()},trigger:"change"}]};return(s,a)=>{const o=B,t=H,d=J,f=k;return D(),N("main",null,[v("div",L,[v("div",P,q(l.value?"开店":"修改店铺信息"),1),r(f,{model:e.value,ref_key:"formDataRef",ref:I,rules:U,class:"form","label-position":"top","status-icon":""},{default:i(()=>[r(o,{label:"请上传你的商店封面",prop:"cover"},{default:i(()=>[r(O,{onUploadImage:R,imageUrl:l.value?"":_.value,update:!l.value},null,8,["imageUrl","update"])]),_:1}),r(o,{label:"请输入你的商店名称",prop:"name"},{default:i(()=>[r(t,{modelValue:e.value.name,"onUpdate:modelValue":a[0]||(a[0]=u=>e.value.name=u),type:"text",clearable:""},null,8,["modelValue"])]),_:1}),r(o,{label:"请选择你的发货区县",prop:"selectedRegion"},{default:i(()=>[r(d,{options:F(V),modelValue:e.value.selectedRegion,"onUpdate:modelValue":a[1]||(a[1]=u=>e.value.selectedRegion=u)},null,8,["options","modelValue"])]),_:1}),r(o,{prop:"detailAddress",label:"请输入详细地址"},{default:i(()=>[r(t,{modelValue:e.value.detailAddress,"onUpdate:modelValue":a[2]||(a[2]=u=>e.value.detailAddress=u),type:"text",clearable:""},null,8,["modelValue"])]),_:1})]),_:1},8,["model"]),v("button",{onClick:S},"确认")])])}}},de=y(Q,[["__scopeId","data-v-4d15b34c"]]);export{de as default};
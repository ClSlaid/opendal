(()=>{"use strict";var e,a,t,c,r,f={},b={};function d(e){var a=b[e];if(void 0!==a)return a.exports;var t=b[e]={exports:{}};return f[e].call(t.exports,t,t.exports,d),t.exports}d.m=f,e=[],d.O=(a,t,c,r)=>{if(!t){var f=1/0;for(i=0;i<e.length;i++){t=e[i][0],c=e[i][1],r=e[i][2];for(var b=!0,o=0;o<t.length;o++)(!1&r||f>=r)&&Object.keys(d.O).every((e=>d.O[e](t[o])))?t.splice(o--,1):(b=!1,r<f&&(f=r));if(b){e.splice(i--,1);var n=c();void 0!==n&&(a=n)}}return a}r=r||0;for(var i=e.length;i>0&&e[i-1][2]>r;i--)e[i]=e[i-1];e[i]=[t,c,r]},d.n=e=>{var a=e&&e.__esModule?()=>e.default:()=>e;return d.d(a,{a:a}),a},t=Object.getPrototypeOf?e=>Object.getPrototypeOf(e):e=>e.__proto__,d.t=function(e,c){if(1&c&&(e=this(e)),8&c)return e;if("object"==typeof e&&e){if(4&c&&e.__esModule)return e;if(16&c&&"function"==typeof e.then)return e}var r=Object.create(null);d.r(r);var f={};a=a||[null,t({}),t([]),t(t)];for(var b=2&c&&e;"object"==typeof b&&!~a.indexOf(b);b=t(b))Object.getOwnPropertyNames(b).forEach((a=>f[a]=()=>e[a]));return f.default=()=>e,d.d(r,f),r},d.d=(e,a)=>{for(var t in a)d.o(a,t)&&!d.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:a[t]})},d.f={},d.e=e=>Promise.all(Object.keys(d.f).reduce(((a,t)=>(d.f[t](e,a),a)),[])),d.u=e=>"assets/js/"+({53:"935f2afb",217:"b56f9725",380:"ab10c306",455:"07df3158",520:"069a7000",533:"b2b675dd",716:"a0405932",1011:"cc62039a",1163:"434bbc94",1372:"1db64337",1477:"b2f554cd",1652:"8cc9519e",1713:"a7023ddc",1846:"054cd2fb",2379:"d5c31a96",2535:"814f3328",2804:"2cbf1097",3057:"136e1c50",3085:"1f391b9e",3089:"a6aa9e1f",3608:"9e4087bc",3845:"e892cec6",4013:"01a85c17",4157:"283e63f8",4195:"c4f5d8e4",4477:"1957547a",4536:"22d8fa67",4538:"f1ca5a39",4719:"8f4159f2",4832:"ece86388",5214:"bb5b2057",5555:"37896f57",6089:"62a401e9",6103:"ccc49370",6427:"5a65e608",6611:"209227ae",6948:"e19a6781",6983:"ae4554eb",7289:"c6ab695b",7757:"57a16c1d",7763:"09cd8fb6",7794:"02514dc9",7918:"17896441",8610:"6875c492",8864:"5f123d4e",9319:"4b02d014",9514:"1be78505",9817:"14eb3368"}[e]||e)+"."+{53:"8d81c32f",217:"fa44f1ff",380:"49c94c36",455:"2937ed05",520:"56d2bde2",533:"035f3452",716:"51d9fc2c",1011:"fb33b112",1163:"bbc02de7",1372:"4d9bd45e",1477:"1bd74887",1506:"7a372cde",1652:"f015c0ec",1713:"bb7c2af2",1846:"6649c26d",2379:"f90b768c",2529:"ea72b266",2535:"fa054466",2804:"3725135d",3057:"b038c2fb",3085:"fc747d21",3089:"66019f66",3608:"822525bd",3845:"b24f404e",4013:"4316cd6e",4157:"23f1d188",4195:"f13ff698",4477:"cb10082f",4536:"5a6481b1",4538:"5b15cc43",4719:"0b2b59d3",4832:"728e2c88",4972:"f7d54065",5214:"a14b3622",5555:"fc5a447b",6089:"f9a2bd67",6103:"ba4b09b5",6427:"55623d5f",6611:"ca89d422",6948:"8576bf3f",6983:"b41e53d9",7289:"048217d0",7757:"0b4010e7",7763:"834644b2",7794:"ad87a856",7918:"fa99442c",8610:"a3ea2b2f",8864:"d44f927d",9319:"e083bd29",9514:"d888abe5",9817:"5d4ade20"}[e]+".js",d.miniCssF=e=>{},d.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),d.o=(e,a)=>Object.prototype.hasOwnProperty.call(e,a),c={},r="opendal-website:",d.l=(e,a,t,f)=>{if(c[e])c[e].push(a);else{var b,o;if(void 0!==t)for(var n=document.getElementsByTagName("script"),i=0;i<n.length;i++){var u=n[i];if(u.getAttribute("src")==e||u.getAttribute("data-webpack")==r+t){b=u;break}}b||(o=!0,(b=document.createElement("script")).charset="utf-8",b.timeout=120,d.nc&&b.setAttribute("nonce",d.nc),b.setAttribute("data-webpack",r+t),b.src=e),c[e]=[a];var l=(a,t)=>{b.onerror=b.onload=null,clearTimeout(s);var r=c[e];if(delete c[e],b.parentNode&&b.parentNode.removeChild(b),r&&r.forEach((e=>e(t))),a)return a(t)},s=setTimeout(l.bind(null,void 0,{type:"timeout",target:b}),12e4);b.onerror=l.bind(null,b.onerror),b.onload=l.bind(null,b.onload),o&&document.head.appendChild(b)}},d.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},d.p="/",d.gca=function(e){return e={17896441:"7918","935f2afb":"53",b56f9725:"217",ab10c306:"380","07df3158":"455","069a7000":"520",b2b675dd:"533",a0405932:"716",cc62039a:"1011","434bbc94":"1163","1db64337":"1372",b2f554cd:"1477","8cc9519e":"1652",a7023ddc:"1713","054cd2fb":"1846",d5c31a96:"2379","814f3328":"2535","2cbf1097":"2804","136e1c50":"3057","1f391b9e":"3085",a6aa9e1f:"3089","9e4087bc":"3608",e892cec6:"3845","01a85c17":"4013","283e63f8":"4157",c4f5d8e4:"4195","1957547a":"4477","22d8fa67":"4536",f1ca5a39:"4538","8f4159f2":"4719",ece86388:"4832",bb5b2057:"5214","37896f57":"5555","62a401e9":"6089",ccc49370:"6103","5a65e608":"6427","209227ae":"6611",e19a6781:"6948",ae4554eb:"6983",c6ab695b:"7289","57a16c1d":"7757","09cd8fb6":"7763","02514dc9":"7794","6875c492":"8610","5f123d4e":"8864","4b02d014":"9319","1be78505":"9514","14eb3368":"9817"}[e]||e,d.p+d.u(e)},(()=>{var e={1303:0,532:0};d.f.j=(a,t)=>{var c=d.o(e,a)?e[a]:void 0;if(0!==c)if(c)t.push(c[2]);else if(/^(1303|532)$/.test(a))e[a]=0;else{var r=new Promise(((t,r)=>c=e[a]=[t,r]));t.push(c[2]=r);var f=d.p+d.u(a),b=new Error;d.l(f,(t=>{if(d.o(e,a)&&(0!==(c=e[a])&&(e[a]=void 0),c)){var r=t&&("load"===t.type?"missing":t.type),f=t&&t.target&&t.target.src;b.message="Loading chunk "+a+" failed.\n("+r+": "+f+")",b.name="ChunkLoadError",b.type=r,b.request=f,c[1](b)}}),"chunk-"+a,a)}},d.O.j=a=>0===e[a];var a=(a,t)=>{var c,r,f=t[0],b=t[1],o=t[2],n=0;if(f.some((a=>0!==e[a]))){for(c in b)d.o(b,c)&&(d.m[c]=b[c]);if(o)var i=o(d)}for(a&&a(t);n<f.length;n++)r=f[n],d.o(e,r)&&e[r]&&e[r][0](),e[r]=0;return d.O(i)},t=self.webpackChunkopendal_website=self.webpackChunkopendal_website||[];t.forEach(a.bind(null,0)),t.push=a.bind(null,t.push.bind(t))})()})();
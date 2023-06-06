"use strict";(self.webpackChunkopendal_website=self.webpackChunkopendal_website||[]).push([[6611],{3905:(e,t,a)=>{a.d(t,{Zo:()=>s,kt:()=>f});var r=a(7294);function n(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function l(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,r)}return a}function o(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?l(Object(a),!0).forEach((function(t){n(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):l(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function i(e,t){if(null==e)return{};var a,r,n=function(e,t){if(null==e)return{};var a,r,n={},l=Object.keys(e);for(r=0;r<l.length;r++)a=l[r],t.indexOf(a)>=0||(n[a]=e[a]);return n}(e,t);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(r=0;r<l.length;r++)a=l[r],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(n[a]=e[a])}return n}var c=r.createContext({}),p=function(e){var t=r.useContext(c),a=t;return e&&(a="function"==typeof e?e(t):o(o({},t),e)),a},s=function(e){var t=p(e.components);return r.createElement(c.Provider,{value:t},e.children)},u="mdxType",h={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var a=e.components,n=e.mdxType,l=e.originalType,c=e.parentName,s=i(e,["components","mdxType","originalType","parentName"]),u=p(a),d=n,f=u["".concat(c,".").concat(d)]||u[d]||h[d]||l;return a?r.createElement(f,o(o({ref:t},s),{},{components:a})):r.createElement(f,o({ref:t},s))}));function f(e,t){var a=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var l=a.length,o=new Array(l);o[0]=d;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[u]="string"==typeof e?e:n,o[1]=i;for(var p=2;p<l;p++)o[p]=a[p];return r.createElement.apply(null,o)}return r.createElement.apply(null,a)}d.displayName="MDXCreateElement"},3542:(e,t,a)=>{a.r(t),a.d(t,{contentTitle:()=>o,default:()=>u,frontMatter:()=>l,metadata:()=>i,toc:()=>c});var r=a(7462),n=(a(7294),a(3905));const l={title:"Download"},o="Download",i={type:"mdx",permalink:"/download",source:"@site/src/pages/download.md",title:"Download",description:"Apache OpenDAL(incubating) is released as a source artifact.",frontMatter:{title:"Download"}},c=[{value:"Releases",id:"releases",level:2},{value:"Notes",id:"notes",level:3},{value:"To verify the signature of the release artifact",id:"to-verify-the-signature-of-the-release-artifact",level:3},{value:"To verify the checksum of the release artifact",id:"to-verify-the-checksum-of-the-release-artifact",level:3}],p={toc:c},s="wrapper";function u(e){let{components:t,...a}=e;return(0,n.kt)(s,(0,r.Z)({},p,a,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"download"},"Download"),(0,n.kt)("br",null),(0,n.kt)("h1",{id:"apache-opendalincubating-downloads"},"Apache OpenDAL(incubating) Downloads"),(0,n.kt)("p",null,"Apache OpenDAL(incubating) is released as a source artifact."),(0,n.kt)("h2",{id:"releases"},"Releases"),(0,n.kt)("table",null,(0,n.kt)("thead",{parentName:"table"},(0,n.kt)("tr",{parentName:"thead"},(0,n.kt)("th",{parentName:"tr",align:null},"Name"),(0,n.kt)("th",{parentName:"tr",align:null},"Archive"),(0,n.kt)("th",{parentName:"tr",align:null},"Signature"),(0,n.kt)("th",{parentName:"tr",align:null},"Checksum"))),(0,n.kt)("tbody",{parentName:"table"},(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:null},"0.36.0-incubating"),(0,n.kt)("td",{parentName:"tr",align:null},(0,n.kt)("a",{parentName:"td",href:"https://dlcdn.apache.org/incubator/opendal/0.36.0/apache-opendal-incubating-0.36.0-src.tar.gz"},"tarball")),(0,n.kt)("td",{parentName:"tr",align:null},(0,n.kt)("a",{parentName:"td",href:"https://dlcdn.apache.org/incubator/opendal/0.36.0/apache-opendal-incubating-0.36.0-src.tar.gz.asc"},"asc")),(0,n.kt)("td",{parentName:"tr",align:null},(0,n.kt)("a",{parentName:"td",href:"https://dlcdn.apache.org/incubator/opendal/0.36.0/apache-opendal-incubating-0.36.0-src.tar.gz.sha512"},"sha512"))))),(0,n.kt)("p",null,"For older releases, please check the ",(0,n.kt)("a",{parentName:"p",href:"https://dlcdn.apache.org/incubator/opendal/"},"archive"),"."),(0,n.kt)("h3",{id:"notes"},"Notes"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},(0,n.kt)("p",{parentName:"li"},"When downloading a release, please check the SHA-512 and verify the OpenPGP compatible signature from the main Apache site. Links are provided above (next to the release download link).")),(0,n.kt)("li",{parentName:"ul"},(0,n.kt)("p",{parentName:"li"},"The KEYS file contains the public keys used for signing release. It is recommended that (when possible) a web of trust is used to confirm the identity of these keys.")),(0,n.kt)("li",{parentName:"ul"},(0,n.kt)("p",{parentName:"li"},"Please download the ",(0,n.kt)("a",{parentName:"p",href:"https://dlcdn.apache.org/incubator/opendal/KEYS"},"KEYS")," as well as the .asc signature files."))),(0,n.kt)("h3",{id:"to-verify-the-signature-of-the-release-artifact"},"To verify the signature of the release artifact"),(0,n.kt)("p",null,"You will need to download both the release artifact and the .asc signature file for that artifact. Then verify the signature using:"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Download the KEYS file and the .asc signature files for the relevant release artifacts."),(0,n.kt)("li",{parentName:"ul"},"Import the KEYS file to your GPG keyring:")),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre"},"$ gpg --import KEYS\n")),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Verify the signature of the release artifact using the following command:")),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre"},"$ gpg --verify <artifact>.asc <artifact>\n")),(0,n.kt)("h3",{id:"to-verify-the-checksum-of-the-release-artifact"},"To verify the checksum of the release artifact"),(0,n.kt)("p",null,"You will need to download both the release artifact and the .sha512 checksum file for that artifact. Then verify the checksum using:"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre"},"$ shasum -a 512 -c <artifact>.sha512\n")))}u.isMDXComponent=!0}}]);
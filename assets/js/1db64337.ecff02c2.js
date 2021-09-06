(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[372],{9082:function(e,t,r){"use strict";r.r(t),r.d(t,{frontMatter:function(){return l},contentTitle:function(){return c},metadata:function(){return s},toc:function(){return u},default:function(){return v}});var n=r(2122),o=r(9756),a=(r(7294),r(3905)),i=["components"],l={},c="Overview",s={unversionedId:"overview",id:"overview",isDocsHomePage:!1,title:"Overview",description:"The wallet library is a stateful package with a standardized interface for developers to build applications involving IOTA value transactions. It provides abstractions to handle IOTA payments and can optionally interact with IOTA Stronghold enclave for seed handling, seed storage and state backup.",source:"@site/docs/overview.md",sourceDirName:".",slug:"/overview",permalink:"/docs/overview",editUrl:"https://github.com/iotaledger/wallet.rs/tree/dev/documentation/docs/overview.md",version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Welcome",permalink:"/docs/welcome"},next:{title:"IOTA Wallet Libraries",permalink:"/docs/libraries/overview"}},u=[{value:"High Level Layered Overview",id:"high-level-layered-overview",children:[]}],p={toc:u};function v(e){var t=e.components,l=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,n.Z)({},p,l,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"overview"},"Overview"),(0,a.kt)("p",null,"The wallet library is a stateful package with a standardized interface for developers to build applications involving IOTA value transactions. It provides abstractions to handle IOTA payments and can optionally interact with ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/iotaledger/stronghold.rs/"},"IOTA Stronghold enclave")," for seed handling, seed storage and state backup. "),(0,a.kt)("p",null,"See the full specification ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/iotaledger/wallet.rs/blob/dev/specs/wallet-ENGINEERING-SPEC-0000.md"},"here"),"."),(0,a.kt)("h2",{id:"high-level-layered-overview"},"High Level Layered Overview"),(0,a.kt)("p",null,(0,a.kt)("img",{alt:"High Level Layered Overview",src:r(4417).Z})))}v.isMDXComponent=!0},3905:function(e,t,r){"use strict";r.d(t,{Zo:function(){return u},kt:function(){return d}});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),s=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=s(e.components);return n.createElement(c.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},v=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),v=s(r),d=o,f=v["".concat(c,".").concat(d)]||v[d]||p[d]||a;return r?n.createElement(f,i(i({ref:t},u),{},{components:r})):n.createElement(f,i({ref:t},u))}));function d(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=v;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:o,i[1]=l;for(var s=2;s<a;s++)i[s]=r[s];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}v.displayName="MDXCreateElement"},4417:function(e,t,r){"use strict";t.Z=r.p+"assets/images/iota_layers_overview-ace8c074803fbef3a53399e12a4c97e6.svg"}}]);
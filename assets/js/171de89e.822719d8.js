"use strict";(self.webpackChunknebula_docs=self.webpackChunknebula_docs||[]).push([[4781],{88:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>u,contentTitle:()=>l,default:()=>p,frontMatter:()=>o,metadata:()=>c,toc:()=>d});var r=t(6070),a=t(1503),s=t(32),i=t(451);const o={},l="Backbone Configuration Guide",c={id:"config/backbone",title:"Backbone Configuration Guide",description:"The Backbone server is the core component of Nebula, responsible for storing encrypted secrets and managing global parameters required for Attribute-Based Encryption (ABE).",source:"@site/versioned_docs/version-0.0.6/config/backbone.mdx",sourceDirName:"config",slug:"/config/backbone",permalink:"/nebula/docs/config/backbone",draft:!1,unlisted:!1,editUrl:"https://github.com/CremitHQ/nebula/tree/main/docs/docs/versioned_docs/version-0.0.6/config/backbone.mdx",tags:[],version:"0.0.6",frontMatter:{},sidebar:"docsSidebar",previous:{title:"Secret",permalink:"/nebula/docs/cli/command/secret"},next:{title:"Authorization server",permalink:"/nebula/docs/config/authorization"}},u={},d=[{value:"Configuration File Overview",id:"configuration-file-overview",level:2},{value:"Configuration Details",id:"configuration-details",level:2},{value:"Port",id:"port",level:3},{value:"JWKs URL Configuration",id:"jwks-url-configuration",level:3},{value:"(Optional) JWKs Refresh Interval",id:"optional-jwks-refresh-interval",level:3},{value:"Database Configuration",id:"database-configuration",level:3},{value:"Host",id:"host",level:4},{value:"Port",id:"port-1",level:4},{value:"Database Name",id:"database-name",level:4},{value:"Authentication Method",id:"authentication-method",level:4},{value:"Username",id:"username",level:4},{value:"Password",id:"password",level:4},{value:"Username",id:"username-1",level:4},{value:"CORS Configuration",id:"cors-configuration",level:3}];function h(e){const n={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",h4:"h4",header:"header",hr:"hr",li:"li",p:"p",pre:"pre",ul:"ul",...(0,a.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.header,{children:(0,r.jsx)(n.h1,{id:"backbone-configuration-guide",children:"Backbone Configuration Guide"})}),"\n",(0,r.jsx)(n.p,{children:"The Backbone server is the core component of Nebula, responsible for storing encrypted secrets and managing global parameters required for Attribute-Based Encryption (ABE)."}),"\n",(0,r.jsx)(n.p,{children:"This guide provides a detailed explanation of the Backbone configuration file, enabling you to tailor Nebula to your specific environment and requirements."}),"\n",(0,r.jsx)(n.h2,{id:"configuration-file-overview",children:"Configuration File Overview"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'port = 8080\njwks_url = "http://authorization:9000/jwks"\n\n[database]\nhost = "db"\nport = 5432\ndatabase_name = "postgres"\nauth.method = "CREDENTIAL"\nauth.username = "postgres"\nauth.password = "password"\n\n'})}),"\n",(0,r.jsxs)(n.p,{children:["This configuration file is written in ",(0,r.jsx)(n.a,{href:"https://toml.io/",children:"TOML"})," format and is divided into several sections."]}),"\n",(0,r.jsx)(n.hr,{}),"\n",(0,r.jsx)(n.h2,{id:"configuration-details",children:"Configuration Details"}),"\n",(0,r.jsx)(n.h3,{id:"port",children:"Port"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"port"})," specifies the port on which the Backbone server listens for incoming requests."]}),"\n",(0,r.jsx)(n.h3,{id:"jwks-url-configuration",children:"JWKs URL Configuration"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"jwks_url"})," where the Authorization server exposes its ",(0,r.jsx)(n.a,{href:"https://datatracker.ietf.org/doc/html/rfc7517",children:"JSON Web Key Set (JWKS)"})," endpoint."]}),"\n",(0,r.jsx)(n.admonition,{type:"note",children:(0,r.jsxs)(n.p,{children:["Ensure that the ",(0,r.jsx)(n.code,{children:"jwks_url"})," is accessible from the Backbone server."]})}),"\n",(0,r.jsx)(n.h3,{id:"optional-jwks-refresh-interval",children:"(Optional) JWKs Refresh Interval"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"jwks_refresh_interval"})," specifies the interval (in seconds) at which the server revalidates the JWKs from the authorization server."]}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:["Default: ",(0,r.jsx)(n.code,{children:"10"})," seconds"]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"database-configuration",children:"Database Configuration"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"[database]"})," section configures the connection to the PostgreSQL database used by the Backbone server."]}),"\n",(0,r.jsx)(n.h4,{id:"host",children:"Host"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"host"})," specifies the hostname or IP address of the PostgreSQL database server."]}),"\n",(0,r.jsx)(n.h4,{id:"port-1",children:"Port"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"port"})," specifies the port on which the PostgreSQL database server is listening."]}),"\n",(0,r.jsx)(n.h4,{id:"database-name",children:"Database Name"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"database_name"})," specifies the name of the PostgreSQL database."]}),"\n",(0,r.jsx)(n.h4,{id:"authentication-method",children:"Authentication Method"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"auth.method"})," specifies the authentication method used to connect to the PostgreSQL database. The available options are:"]}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"CREDENTIAL"}),": Uses a username and password for authentication."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"RDS_IAM_AUTH"}),": Uses AWS RDS IAM authentication."]}),"\n"]}),"\n",(0,r.jsxs)(s.A,{children:[(0,r.jsxs)(i.A,{value:"CREDENTIAL",label:"Credential",children:[(0,r.jsx)(n.h4,{id:"username",children:"Username"}),(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"auth.username"})," specifies the username used to authenticate with the PostgreSQL database."]}),(0,r.jsx)(n.h4,{id:"password",children:"Password"}),(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"auth.password"})," specifies the password used to authenticate with the PostgreSQL database."]})]}),(0,r.jsxs)(i.A,{value:"RDS_IAM_AUTH",label:"AWS RDS IAM",children:[(0,r.jsx)(n.h4,{id:"username-1",children:"Username"}),(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"auth.username"})," specifies the username used to authenticate with the PostgreSQL database."]})]})]}),"\n",(0,r.jsx)(n.h3,{id:"cors-configuration",children:"CORS Configuration"}),"\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:"[cors]"})," section allows you to configure Cross-Origin Resource Sharing (CORS) settings for the Backbone server. The available options are:"]}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"ALLOW_ALL"}),": Allows requests from any origin."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"ALLOW_LIST"}),": Allows requests from specific origins. Also, wildcard characters can be used to match multiple origins."]}),"\n"]}),"\n",(0,r.jsxs)(s.A,{children:[(0,r.jsx)(i.A,{value:"ALLOW_ALL",label:"All Origins",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[cors]\ntype = "ALLOW_ALL"\n'})})}),(0,r.jsxs)(i.A,{value:"ALLOW_LIST",label:"List",children:[(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[cors]\ntype = "ALLOW_LIST"\ndomains = ["http://localhost:3000", "https://*.example.com"]\n'})}),(0,r.jsx)(n.admonition,{type:"warning",children:(0,r.jsxs)(n.p,{children:["Wildcard characters should be used only once in the origin URL. (e.g., ",(0,r.jsx)(n.code,{children:"https://*.example.com"}),")."]})})]})]})]})}function p(e={}){const{wrapper:n}={...(0,a.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(h,{...e})}):h(e)}},451:(e,n,t)=>{t.d(n,{A:()=>i});t(758);var r=t(3526);const a={tabItem:"tabItem_rTHA"};var s=t(6070);function i(e){let{children:n,hidden:t,className:i}=e;return(0,s.jsx)("div",{role:"tabpanel",className:(0,r.A)(a.tabItem,i),hidden:t,children:n})}},32:(e,n,t)=>{t.d(n,{A:()=>A});var r=t(758),a=t(3526),s=t(388),i=t(5557),o=t(2531),l=t(2673),c=t(6302),u=t(9378);function d(e){return r.Children.toArray(e).filter((e=>"\n"!==e)).map((e=>{if(!e||(0,r.isValidElement)(e)&&function(e){const{props:n}=e;return!!n&&"object"==typeof n&&"value"in n}(e))return e;throw new Error(`Docusaurus error: Bad <Tabs> child <${"string"==typeof e.type?e.type:e.type.name}>: all children of the <Tabs> component should be <TabItem>, and every <TabItem> should have a unique "value" prop.`)}))?.filter(Boolean)??[]}function h(e){const{values:n,children:t}=e;return(0,r.useMemo)((()=>{const e=n??function(e){return d(e).map((e=>{let{props:{value:n,label:t,attributes:r,default:a}}=e;return{value:n,label:t,attributes:r,default:a}}))}(t);return function(e){const n=(0,c.XI)(e,((e,n)=>e.value===n.value));if(n.length>0)throw new Error(`Docusaurus error: Duplicate values "${n.map((e=>e.value)).join(", ")}" found in <Tabs>. Every value needs to be unique.`)}(e),e}),[n,t])}function p(e){let{value:n,tabValues:t}=e;return t.some((e=>e.value===n))}function f(e){let{queryString:n=!1,groupId:t}=e;const a=(0,i.W6)(),s=function(e){let{queryString:n=!1,groupId:t}=e;if("string"==typeof n)return n;if(!1===n)return null;if(!0===n&&!t)throw new Error('Docusaurus error: The <Tabs> component groupId prop is required if queryString=true, because this value is used as the search param name. You can also provide an explicit value such as queryString="my-search-param".');return t??null}({queryString:n,groupId:t});return[(0,l.aZ)(s),(0,r.useCallback)((e=>{if(!s)return;const n=new URLSearchParams(a.location.search);n.set(s,e),a.replace({...a.location,search:n.toString()})}),[s,a])]}function b(e){const{defaultValue:n,queryString:t=!1,groupId:a}=e,s=h(e),[i,l]=(0,r.useState)((()=>function(e){let{defaultValue:n,tabValues:t}=e;if(0===t.length)throw new Error("Docusaurus error: the <Tabs> component requires at least one <TabItem> children component");if(n){if(!p({value:n,tabValues:t}))throw new Error(`Docusaurus error: The <Tabs> has a defaultValue "${n}" but none of its children has the corresponding value. Available values are: ${t.map((e=>e.value)).join(", ")}. If you intend to show no default tab, use defaultValue={null} instead.`);return n}const r=t.find((e=>e.default))??t[0];if(!r)throw new Error("Unexpected error: 0 tabValues");return r.value}({defaultValue:n,tabValues:s}))),[c,d]=f({queryString:t,groupId:a}),[b,m]=function(e){let{groupId:n}=e;const t=function(e){return e?`docusaurus.tab.${e}`:null}(n),[a,s]=(0,u.Dv)(t);return[a,(0,r.useCallback)((e=>{t&&s.set(e)}),[t,s])]}({groupId:a}),v=(()=>{const e=c??b;return p({value:e,tabValues:s})?e:null})();(0,o.A)((()=>{v&&l(v)}),[v]);return{selectedValue:i,selectValue:(0,r.useCallback)((e=>{if(!p({value:e,tabValues:s}))throw new Error(`Can't select invalid tab value=${e}`);l(e),d(e),m(e)}),[d,m,s]),tabValues:s}}var m=t(1409);const v={tabList:"tabList_htd4",tabItem:"tabItem_fdRH"};var g=t(6070);function x(e){let{className:n,block:t,selectedValue:r,selectValue:i,tabValues:o}=e;const l=[],{blockElementScrollPositionUntilNextRender:c}=(0,s.a_)(),u=e=>{const n=e.currentTarget,t=l.indexOf(n),a=o[t].value;a!==r&&(c(n),i(a))},d=e=>{let n=null;switch(e.key){case"Enter":u(e);break;case"ArrowRight":{const t=l.indexOf(e.currentTarget)+1;n=l[t]??l[0];break}case"ArrowLeft":{const t=l.indexOf(e.currentTarget)-1;n=l[t]??l[l.length-1];break}}n?.focus()};return(0,g.jsx)("ul",{role:"tablist","aria-orientation":"horizontal",className:(0,a.A)("tabs",{"tabs--block":t},n),children:o.map((e=>{let{value:n,label:t,attributes:s}=e;return(0,g.jsx)("li",{role:"tab",tabIndex:r===n?0:-1,"aria-selected":r===n,ref:e=>l.push(e),onKeyDown:d,onClick:u,...s,className:(0,a.A)("tabs__item",v.tabItem,s?.className,{"tabs__item--active":r===n}),children:t??n},n)}))})}function j(e){let{lazy:n,children:t,selectedValue:s}=e;const i=(Array.isArray(t)?t:[t]).filter(Boolean);if(n){const e=i.find((e=>e.props.value===s));return e?(0,r.cloneElement)(e,{className:(0,a.A)("margin-top--md",e.props.className)}):null}return(0,g.jsx)("div",{className:"margin-top--md",children:i.map(((e,n)=>(0,r.cloneElement)(e,{key:n,hidden:e.props.value!==s})))})}function w(e){const n=b(e);return(0,g.jsxs)("div",{className:(0,a.A)("tabs-container",v.tabList),children:[(0,g.jsx)(x,{...n,...e}),(0,g.jsx)(j,{...n,...e})]})}function A(e){const n=(0,m.A)();return(0,g.jsx)(w,{...e,children:d(e.children)},String(n))}},1503:(e,n,t)=>{t.d(n,{R:()=>i,x:()=>o});var r=t(758);const a={},s=r.createContext(a);function i(e){const n=r.useContext(s);return r.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(a):e.components||a:i(e.components),r.createElement(s.Provider,{value:n},e.children)}}}]);
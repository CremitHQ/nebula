"use strict";(self.webpackChunknebula_docs=self.webpackChunknebula_docs||[]).push([[2379],{7605:(e,t,i)=>{i.r(t),i.d(t,{assets:()=>c,contentTitle:()=>a,default:()=>u,frontMatter:()=>r,metadata:()=>o,toc:()=>d});var n=i(6070),s=i(1503);const r={},a="Glossary",o={id:"getting-started/glossary",title:"Glossary",description:"Understanding Nebula and its advanced secret management system requires familiarity with several technical terms. This glossary is designed to clarify these concepts for users of all technical levels. Please suggest any changes or additions.",source:"@site/versioned_docs/version-0.0.6/getting-started/glossary.md",sourceDirName:"getting-started",slug:"/getting-started/glossary",permalink:"/nebula/docs/getting-started/glossary",draft:!1,unlisted:!1,editUrl:"https://github.com/CremitHQ/nebula/tree/main/docs/docs/versioned_docs/version-0.0.6/getting-started/glossary.md",tags:[],version:"0.0.6",frontMatter:{},sidebar:"docsSidebar",previous:{title:"Quick Start",permalink:"/nebula/docs/getting-started/quick-start"},next:{title:"Installation",permalink:"/nebula/docs/cli/install"}},c={},d=[{value:"Attribute",id:"attribute",level:2},{value:"Attribute-Based Encryption (ABE)",id:"attribute-based-encryption-abe",level:2},{value:"Authority",id:"authority",level:2},{value:"Machine Identity",id:"machine-identity",level:2},{value:"Multi-Authority ABE (MA-ABE)",id:"multi-authority-abe-ma-abe",level:2},{value:"Policy",id:"policy",level:2},{value:"Public Key",id:"public-key",level:2},{value:"Secret Engine",id:"secret-engine",level:2},{value:"User Key",id:"user-key",level:2},{value:"Backbone Server",id:"backbone-server",level:2},{value:"Authorization Server",id:"authorization-server",level:2},{value:"Additional Terms",id:"additional-terms",level:2},{value:"End-to-End Encryption (E2EE)",id:"end-to-end-encryption-e2ee",level:3},{value:"JSON Web Token (JWT)",id:"json-web-token-jwt",level:3},{value:"Identity Provider (IdP)",id:"identity-provider-idp",level:3},{value:"Auditable System",id:"auditable-system",level:3}];function l(e){const t={code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",p:"p",strong:"strong",...(0,s.R)(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(t.header,{children:(0,n.jsx)(t.h1,{id:"glossary",children:"Glossary"})}),"\n",(0,n.jsx)(t.p,{children:"Understanding Nebula and its advanced secret management system requires familiarity with several technical terms. This glossary is designed to clarify these concepts for users of all technical levels. Please suggest any changes or additions."}),"\n",(0,n.jsx)(t.h2,{id:"attribute",children:"Attribute"}),"\n",(0,n.jsx)(t.p,{children:"A characteristic or qualification assigned to a user that determines their access rights. Examples include role, department, security clearance level, or project assignment. In Nebula, attributes are fundamental to controlling access to secrets and enforcing policies."}),"\n",(0,n.jsx)(t.h2,{id:"attribute-based-encryption-abe",children:"Attribute-Based Encryption (ABE)"}),"\n",(0,n.jsx)(t.p,{children:"A type of public-key encryption where secrets are encrypted using a set of attributes. Access to decrypt is granted to users or machines whose attributes whose attributes match the encryption policy. ABE allows for fine-grained access control in secret management systems."}),"\n",(0,n.jsx)(t.h2,{id:"authority",children:"Authority"}),"\n",(0,n.jsx)(t.p,{children:"An entity responsible for managing and issuing specific sets of attributes. Multiple authorities can coexist in a system, each managing different attribute domains. In Nebula, authorities issue user keys based on the attributes a user possesses and expose the public keys used for encryption."}),"\n",(0,n.jsx)(t.h2,{id:"machine-identity",children:"Machine Identity"}),"\n",(0,n.jsxs)(t.p,{children:["A unique identifier assigned to machines or services within Nebula. It ensures that only authorized machines can access certain secrets and interact with the system. Machine identities are managed by the ",(0,n.jsx)(t.strong,{children:"Authorization"})," server."]}),"\n",(0,n.jsx)(t.h2,{id:"multi-authority-abe-ma-abe",children:"Multi-Authority ABE (MA-ABE)"}),"\n",(0,n.jsx)(t.p,{children:"An advanced form of ABE where multiple independent authorities manage different sets of attributes. This enables complex and decentralized access control, critical to Nebula's flexible design."}),"\n",(0,n.jsx)(t.h2,{id:"policy",children:"Policy"}),"\n",(0,n.jsxs)(t.p,{children:["In Nebula, a policy is a set of rules defining which combination of attributes is required to access a particular secret. Policies are composed of a set of binary operations on attributes. These operations typically include AND, OR, allowing for complex logical combinations. For example, a policy might be expressed as ",(0,n.jsx)(t.code,{children:'("Developer" AND "ProjectA") OR "SecurityLevel3"'}),", meaning the secret can be accessed by someone who is both a Developer on ProjectA, or by anyone with Security Level 3 clearance."]}),"\n",(0,n.jsx)(t.h2,{id:"public-key",children:"Public Key"}),"\n",(0,n.jsx)(t.p,{children:"A cryptographic key used in Nebula for encrypting secrets. Public keys are securely distributed by authorities and used in the encryption process to ensure that only users with the corresponding user keys can decrypt a secret. Public keys are integral to the ABE process, enabling secure and attribute-based encryption."}),"\n",(0,n.jsx)(t.h2,{id:"secret-engine",children:"Secret Engine"}),"\n",(0,n.jsx)(t.p,{children:"A modular component in Nebula that handles specific categories of secrets, such as API keys, database credentials, or SSH keys. Secret engines also support automated processes like secret rotation, ensuring security without additional user effort. Nebula's secret engines are extensible and can integrate with external systems to meet diverse secret management needs."}),"\n",(0,n.jsx)(t.h2,{id:"user-key",children:"User Key"}),"\n",(0,n.jsx)(t.p,{children:"A private key unique to each user, generated based on their attributes. This key is issued by the Authority Server and is used exclusively to decrypt secrets matching the user\u2019s attributes. User keys are a critical component of maintaining secure, attribute-based access control."}),"\n",(0,n.jsx)(t.h2,{id:"backbone-server",children:"Backbone Server"}),"\n",(0,n.jsx)(t.p,{children:"The core component of Nebula responsible for storing data and managing global parameters required in ABE. It acts as the central repository for encrypted secrets and oversees overall data integrity and consistency across Nebula."}),"\n",(0,n.jsx)(t.h2,{id:"authorization-server",children:"Authorization Server"}),"\n",(0,n.jsx)(t.p,{children:"A critical component in Nebula that handles user identification and authentication through various identity providers (IdPs) and authentication protocols such as SAML and OIDC. The authorization server issues JSON Web Tokens (JWTs) upon successful authentication and manages machine identities to ensure that only authorized entities can interact with the system."}),"\n",(0,n.jsx)(t.h2,{id:"additional-terms",children:"Additional Terms"}),"\n",(0,n.jsx)(t.h3,{id:"end-to-end-encryption-e2ee",children:"End-to-End Encryption (E2EE)"}),"\n",(0,n.jsx)(t.p,{children:"A security mechanism that ensures data is encrypted on the sender's side and only decrypted by the intended recipient. E2EE protects data from interception or unauthorized access during transmission and storage."}),"\n",(0,n.jsx)(t.h3,{id:"json-web-token-jwt",children:"JSON Web Token (JWT)"}),"\n",(0,n.jsx)(t.p,{children:"A compact, URL-safe means of representing claims to be transferred between two parties. In Nebula, JWTs are issued by the Authorization Server upon successful user authentication and are used to securely transmit user identity and attribute information."}),"\n",(0,n.jsx)(t.h3,{id:"identity-provider-idp",children:"Identity Provider (IdP)"}),"\n",(0,n.jsx)(t.p,{children:"An external service that authenticates users and provides identity information to Nebula. Nebula's Authorization Server integrates with various IdPs using protocols like SAML and OIDC to facilitate seamless user authentication and authorization."}),"\n",(0,n.jsx)(t.h3,{id:"auditable-system",children:"Auditable System"}),"\n",(0,n.jsx)(t.p,{children:"This comprehensive logging and monitoring capabilities. This ensures that all access and modifications to secrets can be tracked and reviewed, enhancing security and compliance with regulatory requirements."})]})}function u(e={}){const{wrapper:t}={...(0,s.R)(),...e.components};return t?(0,n.jsx)(t,{...e,children:(0,n.jsx)(l,{...e})}):l(e)}},1503:(e,t,i)=>{i.d(t,{R:()=>a,x:()=>o});var n=i(758);const s={},r=n.createContext(s);function a(e){const t=n.useContext(r);return n.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function o(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:a(e.components),n.createElement(r.Provider,{value:t},e.children)}}}]);
"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[631],{7689:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>c,contentTitle:()=>t,default:()=>h,frontMatter:()=>l,metadata:()=>a,toc:()=>d});var r=s(4848),i=s(8453);const l={sidebar_position:3},t="Valid Anagram",a={id:"easy/valid_anagram",title:"Valid Anagram",description:"Given two strings s and t, return true if t is an anagram of s, and false otherwise.",source:"@site/docs/easy/valid_anagram.md",sourceDirName:"easy",slug:"/easy/valid_anagram",permalink:"/lcdocs/easy/valid_anagram",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"notesSidebar",previous:{title:"Contains Duplicate",permalink:"/lcdocs/easy/contains_duplicate"},next:{title:"Valid Parentheses",permalink:"/lcdocs/easy/valid_parentheses"}},c={},d=[{value:"Constraints",id:"constraints",level:2},{value:"Examples",id:"examples",level:2},{value:"Example 1",id:"example-1",level:3},{value:"Example 2",id:"example-2",level:3},{value:"Notes",id:"notes",level:2},{value:"Conceptual Idea",id:"conceptual-idea",level:3},{value:"Complexity",id:"complexity",level:3},{value:"Time",id:"time",level:4},{value:"Space",id:"space",level:4},{value:"Solution",id:"solution",level:2},{value:"Rust",id:"rust",level:3}];function o(e){const n={blockquote:"blockquote",code:"code",h1:"h1",h2:"h2",h3:"h3",h4:"h4",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,i.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.h1,{id:"valid-anagram",children:"Valid Anagram"}),"\n",(0,r.jsxs)(n.p,{children:["Given two strings ",(0,r.jsx)(n.code,{children:"s"})," and ",(0,r.jsx)(n.code,{children:"t"}),", return ",(0,r.jsx)(n.code,{children:"true"})," if ",(0,r.jsx)(n.code,{children:"t"})," is an anagram of ",(0,r.jsx)(n.code,{children:"s"}),", and ",(0,r.jsx)(n.code,{children:"false"})," otherwise."]}),"\n",(0,r.jsxs)(n.p,{children:["An ",(0,r.jsx)(n.strong,{children:"Anagram"})," is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once."]}),"\n",(0,r.jsx)(n.h2,{id:"constraints",children:"Constraints"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.code,{children:"1 <= s.length, t.length <= 5 * 104"})}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"s"})," and ",(0,r.jsx)(n.code,{children:"t"})," consist of lowercase English letters."]}),"\n"]}),"\n",(0,r.jsx)(n.h2,{id:"examples",children:"Examples"}),"\n",(0,r.jsx)(n.h3,{id:"example-1",children:"Example 1"}),"\n",(0,r.jsxs)(n.blockquote,{children:["\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:'Input: s = "anagram", t = "nagaram"'})," ",(0,r.jsx)("br",{}),"\n",(0,r.jsx)(n.code,{children:"Output: true"})]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"example-2",children:"Example 2"}),"\n",(0,r.jsxs)(n.blockquote,{children:["\n",(0,r.jsxs)(n.p,{children:[(0,r.jsx)(n.code,{children:'Input: s = "rat", t = "car"'})," ",(0,r.jsx)("br",{}),"\n",(0,r.jsx)(n.code,{children:"Output: false"})]}),"\n"]}),"\n",(0,r.jsx)(n.h2,{id:"notes",children:"Notes"}),"\n",(0,r.jsx)(n.h3,{id:"conceptual-idea",children:"Conceptual Idea"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:["As the characters of ",(0,r.jsx)(n.code,{children:"s"})," and ",(0,r.jsx)(n.code,{children:"t"})," are constrained to lowercase English letters, we know there are only 26 possibilities for each letter"]}),"\n",(0,r.jsxs)(n.li,{children:["Check if ",(0,r.jsx)(n.code,{children:"s"})," and ",(0,r.jsx)(n.code,{children:"t"})," are the same length -> return false if not"]}),"\n",(0,r.jsx)(n.li,{children:"Create an array of 26 integer values initialized to 0 (used to count the frequency of each letter in a string)"}),"\n",(0,r.jsxs)(n.li,{children:["Iterate over ",(0,r.jsx)(n.code,{children:"s"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:["Increment the array value for each letter","\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:["i.e. ",(0,r.jsx)(n.code,{children:"a"})," -> ",(0,r.jsx)(n.code,{children:"0th"})," index in the array, ",(0,r.jsx)(n.code,{children:"b"})," -> ",(0,r.jsx)(n.code,{children:"1st"})," index in the array, etc."]}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,r.jsxs)(n.li,{children:["Iterate over ",(0,r.jsx)(n.code,{children:"t"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:"Decremenmt the array value for each letter"}),"\n"]}),"\n"]}),"\n",(0,r.jsx)(n.li,{children:"Check if the count array contains all zeros -> return true"}),"\n",(0,r.jsx)(n.li,{children:"Otherwise, return false"}),"\n",(0,r.jsxs)(n.li,{children:["An anagram will have the same frequency of letters, so, we increment for 1 and decrement for the other.","\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:"Anagram will have result in all zeroes"}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"complexity",children:"Complexity"}),"\n",(0,r.jsx)(n.h4,{id:"time",children:"Time"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:["Let's say ",(0,r.jsx)(n.code,{children:"s"})," has length ",(0,r.jsx)(n.code,{children:"n"})," and ",(0,r.jsx)(n.code,{children:"t"})," has length ",(0,r.jsx)(n.code,{children:"m"})]}),"\n",(0,r.jsxs)(n.li,{children:["First, we iterate over ",(0,r.jsx)(n.code,{children:"s"})," -> O(n)"]}),"\n",(0,r.jsxs)(n.li,{children:["Second, we iterate voer ",(0,r.jsx)(n.code,{children:"t"})," -> O(m)"]}),"\n",(0,r.jsx)(n.li,{children:"Last, we iterate over the count array -> O(26) (const.)"}),"\n",(0,r.jsx)(n.li,{children:"Results in O(n + m + 26) -> O(n + m)"}),"\n"]}),"\n",(0,r.jsx)(n.h4,{id:"space",children:"Space"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:"Only new space we create is the 26 length array for counting -> O(26) -> O(1)"}),"\n"]}),"\n",(0,r.jsx)(n.h2,{id:"solution",children:"Solution"}),"\n",(0,r.jsx)(n.h3,{id:"rust",children:"Rust"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-rust",children:"fn is_anagram(s: String, t: String) -> bool {\n  let mut freq: [i32; 26] = [0; 26];\n  \n  // Base case\n  if s.len() != t.len() {\n    return false;\n  }\n\n  // Increment for `s`\n  for c in s.chars() {\n    let i = c as usize;\n    freq[i - 97] += 1;\n  }\n\n  // Decrement for `t`\n  for c in t.chars() {\n    let i = c as usize;\n    freq[i - 97] -= 1;\n  }\n\n  // Check freqency is zero\n  for f in freq.iter() {\n    if *f != 0 {\n      return false;\n    }\n  }\n\n  return true;\n}\n"})})]})}function h(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(o,{...e})}):o(e)}},8453:(e,n,s)=>{s.d(n,{R:()=>t,x:()=>a});var r=s(6540);const i={},l=r.createContext(i);function t(e){const n=r.useContext(l);return r.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function a(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:t(e.components),r.createElement(l.Provider,{value:n},e.children)}}}]);
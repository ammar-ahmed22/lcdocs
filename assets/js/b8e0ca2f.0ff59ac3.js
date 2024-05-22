"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[170],{9960:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>o,contentTitle:()=>l,default:()=>h,frontMatter:()=>i,metadata:()=>c,toc:()=>a});var s=t(4848),r=t(8453);const i={sidebar_position:2},l="Top K Frequent Elements",c={id:"medium/top_k_frequent_elements",title:"Top K Frequent Elements",description:"Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.",source:"@site/docs/medium/top_k_frequent_elements.md",sourceDirName:"medium",slug:"/medium/top_k_frequent_elements",permalink:"/lcdocs/medium/top_k_frequent_elements",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"notesSidebar",previous:{title:"Group Anagrams",permalink:"/lcdocs/medium/group_anagrams"}},o={},a=[{value:"Constraints",id:"constraints",level:2},{value:"Examples",id:"examples",level:2},{value:"Example 1",id:"example-1",level:3},{value:"Example 2",id:"example-2",level:3},{value:"Notes",id:"notes",level:2},{value:"Conceptual Idea",id:"conceptual-idea",level:3},{value:"Complexity",id:"complexity",level:3},{value:"Time",id:"time",level:4},{value:"Space",id:"space",level:4},{value:"Solution",id:"solution",level:2},{value:"Rust",id:"rust",level:3}];function u(e){const n={blockquote:"blockquote",code:"code",em:"em",h1:"h1",h2:"h2",h3:"h3",h4:"h4",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,r.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.h1,{id:"top-k-frequent-elements",children:"Top K Frequent Elements"}),"\n",(0,s.jsxs)(n.p,{children:["Given an integer array ",(0,s.jsx)(n.code,{children:"nums"})," and an integer ",(0,s.jsx)(n.code,{children:"k"}),", return the ",(0,s.jsxs)(n.em,{children:[(0,s.jsx)(n.code,{children:"k"})," most frequent elements"]}),". You may return the answer in ",(0,s.jsx)(n.strong,{children:"any order"}),"."]}),"\n",(0,s.jsx)(n.h2,{id:"constraints",children:"Constraints"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:(0,s.jsx)(n.code,{children:"1 <= nums.length <= 105"})}),"\n",(0,s.jsx)(n.li,{children:(0,s.jsx)(n.code,{children:"-104 <= nums[i] <= 104"})}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"k"})," is in the range ",(0,s.jsx)(n.code,{children:"[1, the number of unique elements in the array]"}),"."]}),"\n",(0,s.jsxs)(n.li,{children:["It is ",(0,s.jsx)(n.strong,{children:"guaranteed"})," that the answer is ",(0,s.jsx)(n.strong,{children:"unique"}),"."]}),"\n",(0,s.jsxs)(n.li,{children:["Your algorithm's time complexity must be better than ",(0,s.jsx)(n.code,{children:"O(n log n)"}),", where ",(0,s.jsx)(n.code,{children:"n"})," is the array's size."]}),"\n"]}),"\n",(0,s.jsx)(n.h2,{id:"examples",children:"Examples"}),"\n",(0,s.jsx)(n.h3,{id:"example-1",children:"Example 1"}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:[(0,s.jsx)(n.code,{children:"Input: nums = [1,1,1,2,2,3], k = 2"})," ",(0,s.jsx)("br",{}),"\n",(0,s.jsx)(n.code,{children:"Output: [1,2]"})]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"example-2",children:"Example 2"}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:[(0,s.jsx)(n.code,{children:"Input: nums = [1], k = 1"})," ",(0,s.jsx)("br",{}),"\n",(0,s.jsx)(n.code,{children:"Output: [1]"})]}),"\n"]}),"\n",(0,s.jsx)(n.h2,{id:"notes",children:"Notes"}),"\n",(0,s.jsx)(n.h3,{id:"conceptual-idea",children:"Conceptual Idea"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["We can exploit the constraint that the answer is unique","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"This means that each repeated element will have a unique number of occurrences"}),"\n",(0,s.jsxs)(n.li,{children:["We also know that the frequency of elements can't exceed the size of the input array","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"i.e. The maximum number of repetitions for a number is the size of the array (e.g. all 1's)"}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(n.li,{children:"Create a vector the same size as the input array, initialized to an empty Vector"}),"\n",(0,s.jsx)(n.li,{children:"Count the frequency of numbers using a hashmap"}),"\n",(0,s.jsxs)(n.li,{children:["Iterate over the hashmap","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["For each number, use the frequency as the index in the count vector and push the value to that vector","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["i.e. ",(0,s.jsx)(n.code,{children:"count: Vec<Vec<i32>>"}),". 1 occurs once, 2 occurs once. ",(0,s.jsx)(n.code,{children:"count[1].push(1), count[1].push(2)"})]}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,s.jsxs)(n.li,{children:["Iterate over the array in reverse using a while loop and return ",(0,s.jsx)(n.code,{children:"k"})," elements by stopping when result array has length ",(0,s.jsx)(n.code,{children:"k"})]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"complexity",children:"Complexity"}),"\n",(0,s.jsx)(n.h4,{id:"time",children:"Time"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"We iterate over the numbers once to count the frequency -> O(n)"}),"\n",(0,s.jsxs)(n.li,{children:["Iterate over the hashmap (which can be length ",(0,s.jsx)(n.code,{children:"n"})," at the worst case) -> O(n)"]}),"\n",(0,s.jsxs)(n.li,{children:["Iterate over the array (which can be length ",(0,s.jsx)(n.code,{children:"n"})," at the worst case) -> O(n)"]}),"\n",(0,s.jsx)(n.li,{children:"Results in O(n)"}),"\n"]}),"\n",(0,s.jsx)(n.h4,{id:"space",children:"Space"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["Hashmap (worst case length ",(0,s.jsx)(n.code,{children:"n"}),") -> O(n)"]}),"\n",(0,s.jsxs)(n.li,{children:["Array (worst case length ",(0,s.jsx)(n.code,{children:"n"}),") -> O(n)"]}),"\n",(0,s.jsx)(n.li,{children:"Results in O(n)"}),"\n"]}),"\n",(0,s.jsx)(n.h2,{id:"solution",children:"Solution"}),"\n",(0,s.jsx)(n.h3,{id:"rust",children:"Rust"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {\n  let n = nums.len();\n  // Count frequency\n  let mut freq: HashMap<i32, usize> = HashMap::new();\n  for num in nums.iter() {\n    let updated = match freq.get(num) {\n      Some(frequency) => {\n        *frequency + 1\n      },\n      None => {\n        1\n      }\n    };\n    freq.insert(*num, updated);\n  }\n  \n  // Update the count array\n  let mut count: Vec<Vec<i32>> = vec![vec![]; n + 1];\n  for (key, val) in freq.iter() {\n    count[*val].push(*key);\n  }\n\n  // Return the k most frequent elements\n  let mut result: Vec<i32> = vec![];\n  let mut i: usize = n;\n  while result.len() < (k as usize) {\n    for val in count[i].iter() {\n      result.push(*val);\n    }\n    i -= 1;\n  }\n\n  return result;\n}\n"})})]})}function h(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(u,{...e})}):u(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>l,x:()=>c});var s=t(6540);const r={},i=s.createContext(r);function l(e){const n=s.useContext(i);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:l(e.components),s.createElement(i.Provider,{value:n},e.children)}}}]);
"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[610],{6628:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>o,contentTitle:()=>l,default:()=>a,frontMatter:()=>i,metadata:()=>d,toc:()=>c});var s=t(4848),r=t(8453);const i={sidebar_position:6},l="Two Sum II - Input Array Is Sorted",d={id:"medium/two_sum_sorted",title:"Two Sum II - Input Array Is Sorted",description:"Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.",source:"@site/docs/medium/two_sum_sorted.md",sourceDirName:"medium",slug:"/medium/two_sum_sorted",permalink:"/lcdocs/medium/two_sum_sorted",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:6,frontMatter:{sidebar_position:6},sidebar:"notesSidebar",previous:{title:"Longest Consecutive Sequence",permalink:"/lcdocs/medium/longest_consecutive_sequence"},next:{title:"Three Sum",permalink:"/lcdocs/medium/three_sum"}},o={},c=[{value:"Examples",id:"examples",level:2},{value:"Example 1",id:"example-1",level:3},{value:"Example 2",id:"example-2",level:3},{value:"Example 3",id:"example-3",level:3},{value:"Notes",id:"notes",level:2},{value:"Conceptual Idea",id:"conceptual-idea",level:3},{value:"Complexity",id:"complexity",level:3},{value:"Time",id:"time",level:4},{value:"Space",id:"space",level:4},{value:"Solution",id:"solution",level:2},{value:"Rust",id:"rust",level:3}];function u(e){const n={blockquote:"blockquote",code:"code",h1:"h1",h2:"h2",h3:"h3",h4:"h4",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,r.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.h1,{id:"two-sum-ii---input-array-is-sorted",children:"Two Sum II - Input Array Is Sorted"}),"\n",(0,s.jsxs)(n.p,{children:["Given a ",(0,s.jsx)(n.strong,{children:"1-indexed"})," array of integers ",(0,s.jsx)(n.code,{children:"numbers"})," that is already ",(0,s.jsx)(n.strong,{children:"sorted in non-decreasing order"}),", find two numbers such that they add up to a specific ",(0,s.jsx)(n.code,{children:"target"})," number. Let these two numbers be ",(0,s.jsx)(n.code,{children:"numbers[index1]"})," and ",(0,s.jsx)(n.code,{children:"numbers[index2]"})," where ",(0,s.jsx)(n.code,{children:"1 <= index1 < index2 <= numbers.length"}),"."]}),"\n",(0,s.jsxs)(n.p,{children:["Return the indices of the two numbers, ",(0,s.jsx)(n.code,{children:"index1"})," and ",(0,s.jsx)(n.code,{children:"index2"}),", ",(0,s.jsx)(n.strong,{children:"added by one"})," as an integer array ",(0,s.jsx)(n.code,{children:"[index1, index2]"})," of length 2."]}),"\n",(0,s.jsxs)(n.p,{children:["The tests are generated such that there is ",(0,s.jsx)(n.strong,{children:"exactly one solution"}),". You ",(0,s.jsx)(n.strong,{children:"may not"})," use the same element twice."]}),"\n",(0,s.jsx)(n.p,{children:"Your solution must use only constant extra space."}),"\n",(0,s.jsx)(n.h2,{id:"examples",children:"Examples"}),"\n",(0,s.jsx)(n.h3,{id:"example-1",children:"Example 1"}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:[(0,s.jsx)(n.code,{children:"Input: numbers = [2,7,11,15], target = 9"})," ",(0,s.jsx)("br",{}),"\n",(0,s.jsx)(n.code,{children:"Output: [1, 2]"})]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"example-2",children:"Example 2"}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:[(0,s.jsx)(n.code,{children:"Input: numbers = [2,3,4], target = 6"})," ",(0,s.jsx)("br",{}),"\n",(0,s.jsx)(n.code,{children:"Output: [1, 3]"})]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"example-3",children:"Example 3"}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:[(0,s.jsx)(n.code,{children:"Input: numbers = [-1,0], target = -1"})," ",(0,s.jsx)("br",{}),"\n",(0,s.jsx)(n.code,{children:"Output: [1, 2]"})]}),"\n"]}),"\n",(0,s.jsx)(n.h2,{id:"notes",children:"Notes"}),"\n",(0,s.jsx)(n.h3,{id:"conceptual-idea",children:"Conceptual Idea"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"Create 2 pointers, one at the start of the array and one at the end"}),"\n",(0,s.jsxs)(n.li,{children:["Iterate while true (guaranteed solution)","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["On each iteration, add the values at the pointers","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"If it equals target, return the result"}),"\n",(0,s.jsx)(n.li,{children:"If it is less than the target, decrement the right pointer (smaller number)"}),"\n",(0,s.jsx)(n.li,{children:"If it is greater than the target, increment the left pointer (bigger number)"}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"complexity",children:"Complexity"}),"\n",(0,s.jsx)(n.h4,{id:"time",children:"Time"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"O(n)"}),"\n"]}),"\n",(0,s.jsx)(n.h4,{id:"space",children:"Space"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"O(1)"}),"\n"]}),"\n",(0,s.jsx)(n.h2,{id:"solution",children:"Solution"}),"\n",(0,s.jsx)(n.h3,{id:"rust",children:"Rust"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n    let mut l = 0 as usize;\n    let mut r = nums.len() - 1;\n\n    while l < r {\n      let curr = nums[l] + nums[r];\n      if curr < target {\n        l += 1;\n      } else if curr > target {\n        r -= 1;\n      } else {\n        return vec![(l + 1) as i32, (r + 1) as i32]\n      }\n    }\n    return vec![0, 0];\n}\n"})})]})}function a(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(u,{...e})}):u(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>l,x:()=>d});var s=t(6540);const r={},i=s.createContext(r);function l(e){const n=s.useContext(i);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:l(e.components),s.createElement(i.Provider,{value:n},e.children)}}}]);
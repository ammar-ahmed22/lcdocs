"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[81],{966:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>h,contentTitle:()=>a,default:()=>d,frontMatter:()=>r,metadata:()=>s,toc:()=>o});var i=t(4848),l=t(8453);const r={sidebar_position:8},a="Container With Most Water",s={id:"medium/container_with_most_water",title:"Container With Most Water",description:"You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).",source:"@site/docs/medium/container_with_most_water.md",sourceDirName:"medium",slug:"/medium/container_with_most_water",permalink:"/lcdocs/medium/container_with_most_water",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:8,frontMatter:{sidebar_position:8},sidebar:"notesSidebar",previous:{title:"Three Sum",permalink:"/lcdocs/medium/three_sum"},next:{title:"Evaluate Reverse Polish Notation",permalink:"/lcdocs/medium/evaluate_reverse_polish_notation"}},h={},o=[{value:"Examples",id:"examples",level:2},{value:"Example 1",id:"example-1",level:3},{value:"Example 2",id:"example-2",level:3},{value:"Notes",id:"notes",level:2},{value:"Conceptual Idea",id:"conceptual-idea",level:3},{value:"Complexity",id:"complexity",level:3},{value:"Time",id:"time",level:4},{value:"Space",id:"space",level:4},{value:"Solution",id:"solution",level:2},{value:"Rust",id:"rust",level:3}];function c(e){const n={blockquote:"blockquote",code:"code",em:"em",h1:"h1",h2:"h2",h3:"h3",h4:"h4",img:"img",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,l.R)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h1,{id:"container-with-most-water",children:"Container With Most Water"}),"\n",(0,i.jsxs)(n.p,{children:["You are given an integer array ",(0,i.jsx)(n.code,{children:"height"})," of length ",(0,i.jsx)(n.code,{children:"n"}),". There are ",(0,i.jsx)(n.code,{children:"n"})," vertical lines drawn such that the two endpoints of the ",(0,i.jsx)(n.code,{children:"ith"})," line are ",(0,i.jsx)(n.code,{children:"(i, 0)"})," and ",(0,i.jsx)(n.code,{children:"(i, height[i])"}),"."]}),"\n",(0,i.jsx)(n.p,{children:"Find two lines that together with the x-axis form a container, such that the container contains the most water."}),"\n",(0,i.jsxs)(n.p,{children:["Return the ",(0,i.jsx)(n.em,{children:"maximum amount of water a container can store"}),"."]}),"\n",(0,i.jsxs)(n.p,{children:[(0,i.jsx)(n.strong,{children:"Notice"})," that you may not slant the container."]}),"\n",(0,i.jsx)(n.h2,{id:"examples",children:"Examples"}),"\n",(0,i.jsx)(n.h3,{id:"example-1",children:"Example 1"}),"\n",(0,i.jsx)(n.p,{children:(0,i.jsx)(n.img,{alt:"Container With Most Water Example",src:t(3288).A+"",width:"801",height:"383"})}),"\n",(0,i.jsxs)(n.blockquote,{children:["\n",(0,i.jsxs)(n.p,{children:[(0,i.jsx)(n.code,{children:"Input: height = [1,8,6,2,5,4,8,3,7]"})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Output: 49"})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49."})]}),"\n"]}),"\n",(0,i.jsx)(n.h3,{id:"example-2",children:"Example 2"}),"\n",(0,i.jsxs)(n.blockquote,{children:["\n",(0,i.jsxs)(n.p,{children:[(0,i.jsx)(n.code,{children:"Input: height = [1, 1]"})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Output: 1"})]}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"notes",children:"Notes"}),"\n",(0,i.jsx)(n.h3,{id:"conceptual-idea",children:"Conceptual Idea"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Keep track of the max area with a variable"}),"\n",(0,i.jsxs)(n.li,{children:["Initialize two pointers, one at the start and one at the end","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["On each iteration, calculate the area and keep track of the max","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Area = width (pointer separation) * min_height (minimum of the heights of 2 pointer values)"}),"\n"]}),"\n"]}),"\n",(0,i.jsx)(n.li,{children:"If the left value is smaller than the right, increment the left"}),"\n",(0,i.jsx)(n.li,{children:"If the right value is smaller than left, decrement the right"}),"\n",(0,i.jsxs)(n.li,{children:["If they are the same, increment left and decrement right","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"This allows for the max area to be found because we keep the largest height for as long as possible"}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,i.jsx)(n.h3,{id:"complexity",children:"Complexity"}),"\n",(0,i.jsx)(n.h4,{id:"time",children:"Time"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Only iterate once -> O(n)"}),"\n"]}),"\n",(0,i.jsx)(n.h4,{id:"space",children:"Space"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"No new space -> O(1)"}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"solution",children:"Solution"}),"\n",(0,i.jsx)(n.h3,{id:"rust",children:"Rust"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-rust",children:"fn calc_area(l_ptr: usize, l_val: i32, r_ptr: usize, r_val: i32) -> i32 {\n  let height = std::cmp::min(l_val, r_val);\n  let width = (r_ptr - l_ptr) as i32;\n  return width * height;\n}\n\n// Problem question\nfn max_area(height: Vec<i32>) -> i32 {\n  let mut max = 0;\n  let mut l = 0 as usize;\n  let mut r = height.len() - 1;\n\n  while l < r {\n    let l_val = height[l];\n    let r_val = height[r];\n    max = std::cmp::max(max, calc_area(l, l_val, r, r_val));\n    if l_val == r_val {\n      r -= 1;\n      l += 1;\n    } else if l_val < r_val {\n      l += 1;\n    } else {\n      r -= 1;\n    }\n   }\n  return max;\n}\n"})})]})}function d(e={}){const{wrapper:n}={...(0,l.R)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(c,{...e})}):c(e)}},3288:(e,n,t)=>{t.d(n,{A:()=>i});const i=t.p+"assets/images/container_most_water-9756eeab7f860311b8e964d27a8cf353.jpeg"},8453:(e,n,t)=>{t.d(n,{R:()=>a,x:()=>s});var i=t(6540);const l={},r=i.createContext(l);function a(e){const n=i.useContext(r);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function s(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(l):e.components||l:a(e.components),i.createElement(r.Provider,{value:n},e.children)}}}]);
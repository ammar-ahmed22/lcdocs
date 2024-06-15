"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[408],{2947:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>o,contentTitle:()=>l,default:()=>h,frontMatter:()=>r,metadata:()=>a,toc:()=>c});var i=t(4848),s=t(8453);const r={sidebar_position:9},l="Evaluate Reverse Polish Notation",a={id:"medium/evaluate_reverse_polish_notation",title:"Evaluate Reverse Polish Notation",description:"You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.",source:"@site/docs/medium/evaluate_reverse_polish_notation.md",sourceDirName:"medium",slug:"/medium/evaluate_reverse_polish_notation",permalink:"/lcdocs/medium/evaluate_reverse_polish_notation",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:9,frontMatter:{sidebar_position:9},sidebar:"notesSidebar",previous:{title:"Container With Most Water",permalink:"/lcdocs/medium/container_with_most_water"}},o={},c=[{value:"Examples",id:"examples",level:2},{value:"Example 1",id:"example-1",level:3},{value:"Example 2",id:"example-2",level:3},{value:"Notes",id:"notes",level:2},{value:"Conceptual Idea",id:"conceptual-idea",level:3},{value:"Complexity",id:"complexity",level:3},{value:"Solution",id:"solution",level:2},{value:"Rust",id:"rust",level:3}];function d(e){const n={a:"a",blockquote:"blockquote",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,s.R)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h1,{id:"evaluate-reverse-polish-notation",children:"Evaluate Reverse Polish Notation"}),"\n",(0,i.jsxs)(n.p,{children:["You are given an array of strings ",(0,i.jsx)(n.code,{children:"tokens"})," that represents an arithmetic expression in a ",(0,i.jsx)(n.a,{href:"http://en.wikipedia.org/wiki/Reverse_Polish_notation",children:"Reverse Polish Notation"}),"."]}),"\n",(0,i.jsx)(n.p,{children:"Evaluate the expression. Return an integer that represents the value of the expression."}),"\n",(0,i.jsx)(n.p,{children:"Note that:"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["The valid operators are ",(0,i.jsx)(n.code,{children:"'+'"}),", ",(0,i.jsx)(n.code,{children:"'-'"}),", ",(0,i.jsx)(n.code,{children:"'*'"}),", and ",(0,i.jsx)(n.code,{children:"'/'"}),"."]}),"\n",(0,i.jsx)(n.li,{children:"Each operand may be an integer or another expression."}),"\n",(0,i.jsx)(n.li,{children:"The division between two integers always truncates toward zero."}),"\n",(0,i.jsx)(n.li,{children:"There will not be any division by zero."}),"\n",(0,i.jsx)(n.li,{children:"The input represents a valid arithmetic expression in a reverse polish notation."}),"\n",(0,i.jsx)(n.li,{children:"The answer and all the intermediate calculations can be represented in a 32-bit integer."}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"examples",children:"Examples"}),"\n",(0,i.jsx)(n.h3,{id:"example-1",children:"Example 1"}),"\n",(0,i.jsxs)(n.blockquote,{children:["\n",(0,i.jsxs)(n.p,{children:[(0,i.jsx)(n.code,{children:'tokens = ["2","1","+","3","*"]'})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Output: 9"})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Explanation: ((2 + 1) * 3) = 9"})]}),"\n"]}),"\n",(0,i.jsx)(n.h3,{id:"example-2",children:"Example 2"}),"\n",(0,i.jsxs)(n.blockquote,{children:["\n",(0,i.jsxs)(n.p,{children:[(0,i.jsx)(n.code,{children:'tokens = ["4","13","5","/","+"]'})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Output: 6"})," ",(0,i.jsx)("br",{}),"\n",(0,i.jsx)(n.code,{children:"Explanation: (4 + (13 / 5)) = 6"})]}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"notes",children:"Notes"}),"\n",(0,i.jsx)(n.h3,{id:"conceptual-idea",children:"Conceptual Idea"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Use a stack"}),"\n",(0,i.jsxs)(n.li,{children:["Create a function that can evaluate the 2 numbers with the 4 operands where order matters","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["e.g ",(0,i.jsx)(n.code,{children:"fn eval(int a, int b, char operand) -> int"})]}),"\n",(0,i.jsxs)(n.li,{children:["running this with ",(0,i.jsx)(n.code,{children:"'-'"})," should yield ",(0,i.jsx)(n.code,{children:"a - b"})]}),"\n"]}),"\n"]}),"\n",(0,i.jsxs)(n.li,{children:["Iterate over the tokens","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Whenever it is a number, push to the stack"}),"\n",(0,i.jsxs)(n.li,{children:["When it is an operand","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Pop the last 2 elements from the stack (there should always be atleast 2)"}),"\n",(0,i.jsx)(n.li,{children:(0,i.jsx)(n.code,{children:"let a: int = stack.pop()"})}),"\n",(0,i.jsx)(n.li,{children:(0,i.jsx)(n.code,{children:"let b: int = stack.pop()"})}),"\n",(0,i.jsxs)(n.li,{children:["Evaluate the 2 values with the ",(0,i.jsx)(n.code,{children:"eval"})," function and push the result to the stack"]}),"\n"]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,i.jsx)(n.li,{children:"Return the last element in the stack"}),"\n"]}),"\n",(0,i.jsx)(n.h3,{id:"complexity",children:"Complexity"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Time: O(n)"}),"\n",(0,i.jsx)(n.li,{children:"Space: O(n)"}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"solution",children:"Solution"}),"\n",(0,i.jsx)(n.h3,{id:"rust",children:"Rust"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-rust",children:'fn eval(a: i32, b: i32, op: &String) -> i32 {\n  match op.as_str() {\n    "+" => {\n      return a + b;\n    },\n    "-" => {\n      return a - b;\n    },\n    "/" => {\n      return a / b;\n    },\n    "*" => {\n      return a * b;\n    },\n    _ => {\n      return 0;\n    }\n  }\n}\n\n\nfn eval_rpn(tokens: Vec<String>) -> i32 {\n  let mut stack: Vec<i32> = Vec::new();\n  for char in tokens.iter() {\n    match char.parse::<i32>() {\n      Ok(number) => {\n        stack.push(number);\n      },\n      Err(_) => {\n        let b = stack.pop().unwrap();\n        let a = stack.pop().unwrap();\n        stack.push(eval(a, b, char));\n      } \n    }\n  }\n  return stack[stack.len() - 1];\n}\n'})})]})}function h(e={}){const{wrapper:n}={...(0,s.R)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(d,{...e})}):d(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>l,x:()=>a});var i=t(6540);const s={},r=i.createContext(s);function l(e){const n=i.useContext(r);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function a(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:l(e.components),i.createElement(r.Provider,{value:n},e.children)}}}]);
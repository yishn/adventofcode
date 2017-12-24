// Part 1
I=document.body.innerText.trim().split`\n`.map(x=>x.split`/`),b=(p,V)=>Math.max(...I.map(([x,y],j)=>!V.includes(j)&(x==p|y==p)?+x+ +y+b(x==p?y:x,[...V,j]):0)),b(0,[])

// Part 2
I=document.body.innerText.trim().split`\n`.map(x=>x.split`/`),l=x=>x.length,g=(z,_,a)=>!a.some(u=>l(u)>l(z)),b=(p,V)=>I.map(([x,y],j)=>!V.includes(j)&(x==p|y==p)?[[j],...b(x==p?y:x,[...V,j]).map(z=>[j,...z])].filter(g):[]).reduce((a,x)=>[...a,...x],[]),Math.max(...b(0,[]).filter(g).map(x=>x.reduce((s,i)=>+I[i][0]+ +I[i][1]+s,0)))

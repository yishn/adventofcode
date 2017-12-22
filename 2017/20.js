// Part 1
d=a=>a.reduce((s,x)=>s+(x>0?+x:-x),0),document.body.innerText.trim().split`\n`.map(x=>x.match(/<[^>]*/g).map(y=>y.slice(1).split`,`)).findIndex(([p,,a],_,P)=>P.every(([q,,b])=>d(b)==d(a)&d(q)>=d(p)|d(b)>d(a)))

// Part 2
for(a=(x,y)=>x.map((z,i)=>+y[i]+ +z),P=document.body.innerText.trim().split`\n`.map(x=>x.match(/<[^>]*/g).map(y=>y.slice(1).split`,`)),i=0;i<99;i++)P.map(p=>p[0]=a(p[0],p[1]=a(p[1],p[2]))),P=P.sort().filter((p,i)=>![i-1,i+1].some(j=>P[j]&&p[0]+''==P[j][0]+''));P.length

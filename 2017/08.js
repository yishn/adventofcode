// Part 1
for(p=0,R={},I=document.body.innerText.split`\n`.map(x=>x.split` `),c=a=>eval(a.join``);p<I.length;p++)if(c([R[I[p][4]]|0,...I[p].slice(-2)])){[x,o,y]=I[p];R[x]=(R[x]|0)+y*(o[0]=='i'?1:-1)}Math.max(...Object.values(R))

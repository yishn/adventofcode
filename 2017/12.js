// Part 1
R=[],(d=x=>document.body.innerText.split`\n`[x].split`> `[1].split`, `.map(y=>!R.includes(y)?d(y):0,R.push(x)))('0'),R.length

// Part 2
for(c=0,I=document.body.innerText.trim().split`\n`,R={},d=x=>I[x].split`> `[1].split`, `.map(y=>!R[y]?d(y):0,R[x]=1);(x=I.findIndex((_,i)=>!R[i]))>=0;c++)d(x);c

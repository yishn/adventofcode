// Part 1
R=[],(d=x=>document.body.innerText.split`\n`[x].split`>`[1].split`,`.map(y=>!R.includes(+y)&&d(+y),R.push(x)))(0),R.length

// Part 2
for(c=0,I=document.body.innerText.split`\n`,R={},d=x=>I[x].split`>`[1].split`,`.map(y=>!R[+y]&&d(+y),R[x]=1);(x=I.findIndex((l,i)=>!R[i]&&l))>=0;c++)d(x);c

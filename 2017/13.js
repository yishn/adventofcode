// Part 1
S=(t,r)=>t%(2*r)<r?t%r:r-t%r,document.body.innerText.trim().split`\n`.map(l=>([i,r]=l.split`:`,!S(i,r-1)?i*r:0)).reduce((s,x)=>s+x)

// Part 2
for(I=document.body.innerText.trim().split`\n`.map(l=>l.split`:`),S=(t,r)=>t%(2*r)<r?t%r:r-t%r,d=0;I.some(([i,r])=>!S(+i+d,r-1));)d++;d

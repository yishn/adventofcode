// Part 1
for(p=r=0,R={},I=document.body.innerText.split`\n`,v=x=>+x||R[x]||0;p>=0&p<I.length-1;)[[a,b],x,y]=I[p++].split` `,a=='j'?v(x)&&(p+=v(y)-1):(R[x]=b=='e'?v(y):a=='s'?v(x)-v(y):(r++,v(x)*v(y)));r

// Part 2
for(h=0,b=107900,c=124900;f=0,b<=c;b+=17,f&&h++)for(d=2;d<b;d++)if(!(b%d)){f=1;break}h

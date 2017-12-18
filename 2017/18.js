// Part 1
for(p=s=0,R={},I=document.body.innerText.split`\n`,v=x=>isNaN(x)?R[x]||0:+x;p>=0&p<I.length-1;){[[a,,c],x,y]=I[p++].split` `;if(c=='v'&&v(x)!=0)break;a=='j'?v(x)>0&&(p+=v(y)-1):c=='t'|a=='a'?(R[x]=(a=='a')*v(x)+v(y)):c=='l'?(R[x]=v(x)*v(y)):a=='m'?(R[x]=v(x)%v(y)):(s=v(x))}s


// Part 1
for(p=s=0,R={},I=document.body.innerText.split`\n`,v=x=>isNaN(x)?R[x]||0:+x;p>=0&p<I.length-1;){[[a,,c],x,y]=I[p++].split` `;if(c=='v'&&v(x)!=0)break;a=='j'?v(x)>0&&(p+=v(y)-1):c=='t'|a=='a'?(R[x]=(a=='a')*v(x)+v(y)):c=='l'?(R[x]=v(x)*v(y)):a=='m'?(R[x]=v(x)%v(y)):(s=v(x))}s

// Part 2
for(C=r=p=q=0,P=[],Q=[],R={p:0},S={p:1},I=document.body.innerText.split`\n`;p>=0&p<I.length-1&C<2;)C=0,[[p,P,R],[q,Q,S]].map(([j,P,R],i,A)=>{d=1,v=x=>isNaN(x)?R[x]||0:+x,[[a,,c],x,y]=I[j].split` `;c=='v'?!P.length?C++&d--:(R[x]=P.shift()):a=='j'?v(x)>0&&(d=v(y)):c=='t'|a=='a'?(R[x]=(a=='a')*v(x)+v(y)):c=='l'?(R[x]=v(x)*v(y)):a=='m'?(R[x]=v(x)%v(y)):A[(i+1)%2][1].push(v(x))&&i&&r++;!i?p+=d:q+=d});r

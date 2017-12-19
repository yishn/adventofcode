// Part 1
for(C=r='',M=document.body.innerText.split`\n`.map(x=>[...x]),x=M[0].indexOf('|'),y=e=1,d=0,c=(x,y)=>M[y]&&M[y][x];C!=' ';x+=d,y+=e){for(;(C=c(x,y))!='+'&C!=' ';x+=d,y+=e)r+=C.replace(/\||-/g,'');d=d!=0?0:c(x+1,y)!=' '?1:-1;e=e!=0?0:c(x,y+1)!=' '?1:-1}r

// Part 2
for(M=document.body.innerText.split`\n`.map(x=>[...x]),x=M[0].indexOf('|'),C=y=e=1,r=d=0,c=(x,y)=>M[y]&&M[y][x];C!=' ';x+=d,y+=e){for(;(C=c(x,y))!='+'&C!=' ';x+=d,y+=e)r++;d=d!=0?0:c(x+1,y)!=' '?1:-1;e=e!=0?0:c(x,y+1)!=' '?1:-1;r++}r

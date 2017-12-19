// Part 1
for(C=r='',M=document.body.innerText.split`\n`.map(x=>[...x]),x=M[0].indexOf('|'),y=e=1,d=0,c=(x,y)=>M[y]&&M[y][x];C!=' ';){for(;!'+ '.includes(C=c(x,y));){r+=!'|-'.includes(C)?C:'';x+=d;y+=e}d=d!=0?0:c(x+1,y)!=' '?1:-1;e=e!=0?0:c(x,y+1)!=' '?1:-1;x+=d;y+=e}r

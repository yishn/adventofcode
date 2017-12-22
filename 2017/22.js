// Part 1
M={},r=x=y=dx=0,dy=-1,document.body.innerText.trim().split`\n`.map((r,j)=>[...r].map((v,i)=>M[[i-~~l,j-~~l]]=v=='#',l=r.length/2));for(i=0;i<10000;i++,r+=M[[x,y]]=+!M[[x,y]],x+=dx,y+=dy)v=M[[x,y]]?-1:1,[dx,dy]=[v*dy,-v*dx];r

// Part 2
M={},r=x=y=dx=0,dy=-1,document.body.innerText.trim().split`\n`.map((r,j)=>[...r].map((v,i)=>M[[i-~~l,j-~~l]]=+(v=='#')*2,l=r.length/2));for(i=0;i<10**7;i++,x+=dx,y+=dy)v=+M[[x,y]]|0,v!=1&&([dx,dy]=v==3?[-dx,-dy]:[(1-v)*dy,(v-1)*dx]),M[[x,y]]=++v%4,v==2&&r++;r

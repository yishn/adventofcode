// Part 1
for(p=0,C=[0],i=1;i<2018;)p=(+document.querySelector('.puzzle-input').innerText+p)%i,C.splice(++p,0,i++);C[++p%--i]

// Part 2
for(s=+document.querySelector('.puzzle-input').innerText,r=z=p=i=0;i<5*10**7;)p=(s+p)%++i+1,p<=z?z++:p==z+1&&(r=i);r

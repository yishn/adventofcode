// Part 1
x=y=0,document.body.innerText.split`,`.map(([f,s])=>(x+=!!s*(s=='e'?1:-1),y+=f=='n'&s!='e'?-1:f=='s'&s!='w'?1:0)),X=x<0?-x:x,Y=y<0?-y:y,x*y>0?Math.max(X,Y):X+Y

// Part 2
D=x=y=0,document.body.innerText.split`,`.map(([f,s])=>(x+=!!s*(s=='e'?1:-1),y+=f=='n'&s!='e'?-1:f=='s'&s!='w'?1:0,X=x<0?-x:x,Y=y<0?-y:y,D=Math.max(D,x*y>0?Math.max(X,Y):X+Y))),D

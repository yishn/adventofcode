// Part 1
s=p=0,l=256,K=[...Array(l)].map((_,i)=>i),document.body.innerText.split`,`.map(x=>[...K,...K].slice(p%l,+x+p%l).map((d,i)=>K[(p-i-s)%l]=d,p+=+x+s++)),K[0]*K[1]

// Part 2
for(j=s=p=0,l=256,e=16,K=[...Array(l)].map((_,i)=>i);j++<64;)[...[...document.body.innerText.trim()].map(x=>x.charCodeAt()),17,31,73,47,23].map(x=>[...K,...K].slice(p%l,x+p%l).map((d,i)=>K[(p-i-s)%l]=d,p+=x+s++));for(h='',i=0;d=K.slice(i*e,i++*e+e).reduce((a,x)=>a^x),i<e;)h+=(d<e?0:'')+d.toString(e);h

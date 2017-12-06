// Part 1
for(c=0,s=document.body.innerText.trim().split`\t`,M={};!(s in M);){M[s]=c++;for(i=s.findIndex(x=>x==Math.max(...s)),n=s[i],s[i]=0;n>0;n--)s[++i%s.length]++}c

// Part 2
for(c=0,s=document.body.innerText.trim().split`\t`,M={};!(s in M);){M[s]=c++;for(i=s.findIndex(x=>x==Math.max(...s)),n=s[i],s[i]=0;n>0;n--)s[++i%s.length]++}c-M[s]

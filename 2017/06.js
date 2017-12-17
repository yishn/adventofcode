// Part 1
for(c=0,s=document.body.innerText.split`\t`.map(x=>+x),M={};!M[s];){M[s]=c++;for(i=s.indexOf(Math.max(...s)),n=s[i],s[i]=0;n-->0;)s[++i%s.length]++}c

// Part 2
for(c=0,s=document.body.innerText.split`\t`.map(x=>+x),M={};!M[s];){M[s]=c++;for(i=s.indexOf(Math.max(...s)),n=s[i],s[i]=0;n-->0;)s[++i%s.length]++}c-M[s]

// Part 1
for(s=p=0,a=document.body.innerText.split`\n`;p<a.length-1&p>=0;s++)p+=a[p]++;s

// Part 2
for(s=p=0,a=document.body.innerText.split`\n`;p<a.length-1&p>=0;s++)q=p,p+=+a[p],a[q]-=a[q]>2?1:-1;s

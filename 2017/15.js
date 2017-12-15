// Part 1
for (m=2**31-1,c=n=0,[a,b]=document.body.innerText.split`\n`.map(x=>x&&+x.split` `[4]);n++<4*10**7;){a=a*16807%m;b=b*48271%m;c+=!((a^b)<<16)}c

// Part 2
for (m=2**31-1,c=n=0,[a,b]=document.body.innerText.split`\n`.map(x=>x&&+x.split` `[4]);n++<5*10**6;){while((a=a*16807%m)%4>0)0;while((b=b*48271%m)%8>0)0;c+=!((a^b)<<16)}c

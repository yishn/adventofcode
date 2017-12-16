// Part 1
s='abcdefghijklmnop'.split``,w=(s,i,j)=>([s[i],s[j]]=[s[j],s[i]],s),document.body.innerText.split`,`.map(x=>s=(l=x.slice(1),x[0]=='s'?[...s.slice(-l),...s.slice(0,-l)]:x[0]=='x'?w(s,...l.split`/`):w(s,s.indexOf(x[1]),s.indexOf(x[3])))),s.join``

// Part 2
s='abcdefghijklmnop'.split``,w=(s,i,j)=>([s[i],s[j]]=[s[j],s[i]],s),[...Array(100)].map(_=>document.body.innerText.split`,`.map(x=>s=(l=x.slice(1),x[0]=='s'?[...s.slice(-l),...s.slice(0,-l)]:x[0]=='x'?w(s,...l.split`/`):w(s,s.indexOf(x[1]),s.indexOf(x[3]))))),s.join``

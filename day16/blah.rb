g=(?a..?p).to_a
y=gets.split(?,)
h={} # part 2
c=1000000000
c.times{|x|break if x>=c
	(
		tv,w=h[g*'']
		gap=x-tv
		remain=c-x-1
		(
			    skiptimes=remain/gap
			        c-=skiptimes*gap
		)if remain>=gap
	) if h.has_key?(g*'')

	uu=g*'' # part 2 end
	y.each{|x|case x
	when /s(.+)/
		g.rotate!(-$1.to_i)
	when /x(\d+)\/(\d+)/
		a,b=$1.to_i,$2.to_i
		g[a],g[b]=g[b],g[a]
	when /p(.)\/(.)/
		a,b=g.index($1),g.index($2)
		g[a],g[b]=g[b],g[a]
	end}
	h[uu]=[x,g[0..-1]]} # part 2
p g.join

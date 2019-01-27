u <- rbeta(100000,5,1.5)*100
#hist(u)
p <- c(0.5, 1, 5, 10, 25, 50, 75, 90, 95, 99, 99.5)/100
sta <- c("PCTL0.5","PCTL1","PCTL5","PCTL10","PCTL25","PCTL50",
         "PCTL75","PCTL90","PCTL95","PCTL99","PCTL99.5",
         "mean","var","stdev","count","sum",
         quantile(u,p),
         mean(u),var(u),sd(u),length(u),sum(u))
write(u, file="data/large-skew.dat", ncolumns=1)
write(u[1:50000], file="data/large-skew-chunk1.dat", ncolumns=1)
write(u[50001:100000], file="data/large-skew-chunk2.dat", ncolumns=1)
write(sta, file="data/large-skew.sta", ncolumns=16)
u<-sort(u,decreasing=FALSE)
write(u, file="data/large-skew-asc.dat", ncolumns=1)
u<-sort(u,decreasing=TRUE)
write(u, file="data/large-skew-desc.dat", ncolumns=1)

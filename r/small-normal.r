u <- rnorm(50)*100
#hist(u)
p <- c(0.5, 1, 5, 10, 25, 50, 75, 90, 95, 99, 99.5)/100
sta <- c("PCTL0.5","PCTL1","PCTL5","PCTL10","PCTL25","PCTL50",
         "PCTL75","PCTL90","PCTL95","PCTL99","PCTL99.5",
         "mean","var","stdev","count","sum",
         quantile(u,p),
         mean(u),var(u),sd(u),length(u),sum(u))
write(u, file="data/small-normal.dat", ncolumns=1)
write(u[1:25], file="data/small-normal-chunk1.dat", ncolumns=1)
write(u[26:50], file="data/small-normal-chunk2.dat", ncolumns=1)
write(sta, file="data/small-normal.sta", ncolumns=16)
u<-sort(u,decreasing=FALSE)
write(u, file="data/small-normal-asc.dat", ncolumns=1)
u<-sort(u,decreasing=TRUE)
write(u, file="data/small-normal-desc.dat", ncolumns=1)

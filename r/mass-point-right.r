m <- rep(86.85518, 1500)
u <- runif(8500)*80
c <- c(u, m)
#hist(c)
p <- c(0.5, 1, 5, 10, 25, 50, 75, 90, 95, 99, 99.5)/100
sta <- c("PCTL0.5","PCTL1","PCTL5","PCTL10","PCTL25","PCTL50",
         "PCTL75","PCTL90","PCTL95","PCTL99","PCTL99.5",
         "mean","var","stdev","count","sum",
         quantile(u,p),
         mean(u),var(u),sd(u),length(u),sum(u))
write(u, file="data/mass-point-right.dat", ncolumns=1)
write(u[1:5000], file="data/mass-point-right-chunk1.dat", ncolumns=1)
write(u[5001:10000], file="data/mass-point-right-chunk2.dat", ncolumns=1)
write(sta, file="data/mass-point-right.sta", ncolumns=16)
u<-sort(u,decreasing=FALSE)
write(u, file="data/mass-point-right-asc.dat", ncolumns=1)
u<-sort(u,decreasing=TRUE)
write(u, file="data/mass-point-right-desc.dat", ncolumns=1)

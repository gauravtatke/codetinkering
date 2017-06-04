#include<netinet/in.h>

int main(){
    char buff[64];
    struct sockaddr_in{
        short int sin_family;
        unsigned short int sin_port;
        struct in_addr sin_addr;
    }serv,cli;
    int sfd = socket(AF_INET,SOCK_STREAM,0);
    serv.sin_family = AF_INET;
    serv.sin_addr.s_addr = INADDR_ANY;
    serv.sin_port = htons(54440);
    bind(sfd,&serv,sizeof(serv));
    listen(sfd,2);
    int nsd = accept(sfd,&cli,sizeof(serv));
    int rd = read(nsd,&buff,64);
    write(1,&buff,rd);
}


//引入TCP的库
use std::net::{TcpListener,TcpStream};
//引入创建线程的库
use std::thread;
//引入时间库
use std::time;
//引入io库处理错误
use std::io::{self, Read, Write};

//创建函数handle_client用来处理客户端发送的stream数据
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //创建一个数组buf,长度512
    let mut buf = [0;512];
    //loop循环
    loop {
      //读取stream的内容到buf
        let bytes_read = stream.read(&mut buf)?;
        //如果等于0就没有内容了,return 结束循环
        if bytes_read == 0 {
            return Ok(());
        }
        //打印客户端发送的内容
        println!("Request: {}", String::from_utf8_lossy(&buf[..]));
        //把内容写回去
        stream.write(&buf[..bytes_read])?;
        //睡眠一秒
        thread::sleep(time::Duration::from_secs(1));

    }
}

fn main() -> io::Result<()>{
    //绑定TCP服务的IP和端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //创建线程容器来存放线程的句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //for循环listener的连接
    for stream in listener.incoming() {
        //转换stream,如果有问题提示failed
        let stream = stream.expect("failed");
        //创建线程,用闭包调用handle_client传入stream
        let handle = thread::spawn(move || {
            //调用handle_client 如果错误,打印错误信息
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        //线程push进容器里
        thread_vec.push(handle);
    }
    //for循环等待线程结束
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
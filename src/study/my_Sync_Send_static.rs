// 数据要在线程之间被move需要满足Send trait.
// Rust要求多线程共享的可读写的数据满足Sync trait。
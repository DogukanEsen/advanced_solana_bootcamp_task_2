enum Publication {
    Book(Book),
    Magazine(Magazine)
}

struct Book{
    title:String,
    author:String,
    page_count:u32,
}

struct Magazine{
    title:String,
    issue:u32,
    topic:String,
}

fn print_publications(publications: Vec<Publication>){
    for publication in publications{
        match publication{
            Publication::Book(ref book) =>{
                println!(
                    "Book: {}, author: {}, {} pages",
                    book.title,book.author,book.page_count
                );
            }
            Publication::Magazine(ref magazine) =>{
                println!(
                    "Magazine: {}, Issue: {}, Topic: {}",
                    magazine.title,magazine.issue,magazine.topic
                );
            }
        }
    }
}

fn main(){
    let book0 = Book{
        title:"Beyaz Zambaklar Ülkesinde".to_string(),
        author:"Grigory Petrov".to_string(),
        page_count:230
    };
    let magazine0 = Magazine{
        title:"Advanced Solana Bootcamp".to_string(),
        issue:20,
        topic:"Solana, web3, rust".to_string(),
    };
    let publications=vec![Publication::Book(book0),Publication::Magazine(magazine0)];

    print_publications(publications)
}

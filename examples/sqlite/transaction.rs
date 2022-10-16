use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // 打开第一个例子所创建的数据库
    let mut conn = Connection::open("data/cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}

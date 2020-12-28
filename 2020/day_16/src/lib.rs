#[derive(Clone, Debug)]
pub struct Field {
    name: String,
    min_in: usize,
    min_out: usize,
    max_in: usize,
    max_out: usize,
    position: usize,
}

impl Field {
    pub fn new(
        name: &String,
        min_in: usize,
        min_out: usize,
        max_in: usize,
        max_out: usize,
    ) -> Field {
        Field {
            name: name.to_string(),
            position: 0,
            min_in,
            min_out,
            max_in,
            max_out,
        }
    }

    fn is_valid(&self, value: usize) -> bool {
        (value >= self.min_in && value <= self.min_out)
            || (value >= self.max_out && value <= self.max_in)
    }
}

#[derive(Clone, Debug)]
pub struct Plane {
    ticket_fields: Vec<Field>,
    tickets: Vec<Ticket>,
    valid_tickets: Vec<Ticket>
}

impl Plane {
    pub fn new(ticket_fields: &Vec<Field>, tickets: &Vec<Ticket>) -> Plane {
        Plane {
            ticket_fields: ticket_fields.clone(),
            tickets: tickets.clone(),
            valid_tickets: Vec::new(),
        }
    }

    pub fn invalid_rate(&self) -> usize {
        self.tickets
            .iter()
            .map(|ticket| {
                ticket
                    .fields
                    .iter()
                    .filter(|field| {
                        self.ticket_fields
                            .iter()
                            .all(|ticket| !ticket.is_valid(**field))
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
    
    pub fn filter_tickets(&mut self) {
        let mut valid: Vec<Ticket> = Vec::new();
        self.tickets
        .iter()
        .filter(|ticket| {
            ticket
                .fields
                .iter()
                .all(|field| {
                    self.ticket_fields
                        .iter()
                        .all(|ticket| !ticket.is_valid(*field))
                })
        }).for_each(|ticket|{
            valid.push(ticket.clone());
        });
        self.valid_tickets = valid.clone();
    }

    pub fn define_fields(&mut self) {
        let mut i = 0;
        let mut columns: Vec<(Vec<usize>, usize)> = Vec::new();
        let len = self.ticket_fields.len();
        loop {
            columns.push((Vec::new(), 0));
            i += 1;
            if i == len {
                break;
            }
        }
        self.valid_tickets.iter().for_each(|ticket| {
            ticket.fields.iter().enumerate().for_each(|value| {
                columns[value.0].0.push(*value.1);
                columns[value.0].1 = value.0;
            });
        });
        //println!("{:?}", columns);
        let mut locked: Vec<usize> = Vec::new();
        self.ticket_fields.iter_mut().for_each(|field| {
            i = 0;
            loop {
                if i == len {
                    break;
                }
                if columns[i].0.iter().all(|el| field.is_valid(*el)) && !locked.contains(&i) {
                    field.position = i;
                    locked.push(i);
                    //columns.remove(i);
                    break;
                }
                i += 1;
            }
        });
    }

    pub fn calc_fields(&self) -> usize {
        let mut power = 1;
        self.ticket_fields
            .iter()
            .filter(|&field| field.name.strip_prefix("departure").is_some())
            .for_each(|field| {
                println!("{}-{}", field.name, self.tickets[0].fields[field.position]);
                power *= self.tickets[0].fields[field.position];
            });
        power
    }
}

#[derive(Clone, Debug)]
pub struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    pub fn new(fields: &Vec<usize>) -> Ticket {
        Ticket {
            fields: fields.clone(),
        }
    }
}

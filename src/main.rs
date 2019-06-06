use std::any::Any;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use std::thread;

use std::sync::mpsc::channel;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};


use druid_shell::platform::{WindowBuilder, IdleHandle};
use druid_shell::win_main;

use druid::{UiMain, UiState, Ui, LayoutResult, LayoutCtx, HandlerCtx, Geometry, PaintCtx, Id, BoxConstraints, Widget};

mod python;

struct RemoteWidget {
    src: Option<String>,
}

impl RemoteWidget {
    pub fn new() -> RemoteWidget {
        RemoteWidget {
            src: None,
        }
    }

    pub fn ui(self, ctx: &mut Ui) -> Id {
        ctx.add(self, &[])
    }

    fn do_python_thing(&mut self, paint_ctx: &mut PaintCtx) {
        if let Some(ref code) = self.src {
            match python::run_python(&mut paint_ctx.render_ctx, code) {
                Err(e) => eprintln!("py eval failed {:?}", e),
                Ok(_) => (),
            }
        }
    }
}

impl Widget for RemoteWidget {
    fn layout(
        &mut self,
        bc: &BoxConstraints,
        _children: &[Id],
        _size: Option<(f32, f32)>,
        _ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        LayoutResult::Size(bc.max())
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _geom: &Geometry) {
        self.do_python_thing(paint_ctx);
    }

    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerCtx) -> bool {
        if let Some(string) = payload.downcast_ref::<String>() {
            self.src = string.clone().into();
            ctx.invalidate();
            true
        } else {
            println!("downcast failed");
            false
        }
    }
}

fn main() {

    let to_watch = env::args()
        .skip(1)
        .next()
        .map(PathBuf::from)
        .expect("please pass a path to the python file we're watching.");
    if !to_watch.exists() || !to_watch.is_file() {
        eprintln!("The path {:?} does not exist or is not a file", to_watch);
        std::process::exit(1);
    }

    druid_shell::init();

    let mut run_loop = win_main::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let mut state = UiState::new();

    let coderunner = RemoteWidget::new().ui(&mut state);
    state.set_root(coderunner);
    builder.set_handler(Box::new(UiMain::new(state)));
    builder.set_title("Ext event example");
    let window = builder.build().unwrap();
    let idle_handle = window.get_idle_handle().unwrap();
    watch_file(to_watch, coderunner, idle_handle);

    window.show();
    run_loop.run();
}


fn watch_file(p: PathBuf, widget: Id, handle: IdleHandle) {

    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(200)).expect("failed to create watcher");
        watcher.watch(p, RecursiveMode::NonRecursive).expect("failed to watch file");

        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Write(p)) => {
                    println!("wrote to {:?}", p);
                    let contents = std::fs::read_to_string(p).expect("failed to read file");
                    UiMain::send_ext(&handle, widget, contents);
                }
                Ok(event) => println!("{:?}", event),
                Err(_) => break,
            }
        }
    });
}

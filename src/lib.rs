use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::f32::consts::PI;

const START_DEG: f32 = 120.0;
const SWEEP_DEG: f32 = 300.0;
const DEAD_START: f32 = 60.0;
const DEAD_END: f32 = 120.0;

fn lerp_color(t: f32) -> egui::Color32 {
    let t = t.clamp(0.0,1.0);
    if t <= 0.5 { let k=t*2.0; egui::Color32::from_rgb((34.0+(250.0-34.0)*k)as u8,(197.0+(204.0-197.0)*k)as u8,(94.0+(21.0-94.0)*k)as u8) }
    else { let k=(t-0.5)*2.0; egui::Color32::from_rgb((250.0+(220.0-250.0)*k)as u8,(204.0+(38.0-204.0)*k)as u8,(21.0+(38.0-21.0)*k)as u8) }
}
fn angle_to_pos(c: egui::Pos2, r:f32, a:f32)->egui::Pos2{egui::Pos2::new(c.x+a.cos()*r,c.y+a.sin()*r)}
fn rounded_line(p: &egui::Painter, a: egui::Pos2, b: egui::Pos2, w:f32, col: egui::Color32){
    p.line_segment([a,b], egui::Stroke::new(w,col));
    p.circle_filled(a,w*0.5,col); p.circle_filled(b,w*0.5,col);
}

struct KnobPlugin {
    params: Arc<KnobParams>,
    value: Arc<AtomicF32>,
    current: Arc<AtomicF32>,
    state: Arc<EguiState>,
}
#[derive(Params)]
struct KnobParams {
    #[id="knob"]
    knob: FloatParam,
}

impl Default for KnobPlugin {
    fn default()->Self{
        Self{
            params: Arc::new(KnobParams{
                knob: FloatParam::new("Knob",50.0,FloatRange::Linear{ min:0.0,max:100.0 }).with_unit("%").with_value_to_string(formatters::v2s_f32_percentage(0)).with_string_to_value(formatters::s2v_f32_percentage())
            }),
            value: Arc::new(AtomicF32::new(50.0)),
            current: Arc::new(AtomicF32::new(50.0)),
            state: EguiState::from_size(460,460),
        }
    }
}
impl Plugin for KnobPlugin {
    const NAME: &'static str = "KnobMuse";
    const VENDOR: &'static str = "tayo6";
    const URL: &'static str = "https://tayo6.github.io/knobmus";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout{ main_input_channels: None, main_output_channels: None,..AudioIOLayout::const_default() }];
    type SysExMessage = ();
    type BackgroundTask = ();
    fn params(&self)->Arc<dyn Params>{self.params.clone()}
    fn editor(&mut self, _a: AsyncExecutor<Self>)->Option<Box<dyn Editor>>{
        let params = self.params.clone();
        let value = self.value.clone();
        let current = self.current.clone();
        let state = self.state.clone();
        create_egui_editor(state, (),
            |_,_|{},
            move |egui_ctx, _setter, _state|{
                let dt = egui_ctx.input(|i| i.unstable_dt).clamp(0.0,0.05);
                let tgt = value.load(Ordering::Relaxed);
                let mut cur = current.load(Ordering::Relaxed);
                let l = 1.0 - (-dt*18.0).exp();
                cur += (tgt-cur)*l;
                current.store(cur, Ordering::Relaxed);

                egui::CentralPanel::default().frame(egui::Frame{fill: egui::Color32::WHITE, inner_margin: egui::Margin::symmetric(10.0,10.0),..Default::default()}).show(egui_ctx,|ui|{
                    ui.vertical_centered(|ui|{
                        let desired = egui::Vec2::splat(460.0);
                        let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());
                        let center = rect.center();
                        let radius = 98.0; let sw = 16.0; let tick_r = 128.0;
                        let start_rad = START_DEG*PI/180.0;
                        let sweep_rad = SWEEP_DEG*PI/180.0;
                        let cur_t = cur/100.0;
                        let cur_rad = start_rad + cur_t*sweep_rad;
                        let cur_col = lerp_color(cur_t);

                        if resp.dragged(){
                            if let Some(p)=resp.interact_pointer_pos(){
                                let v=p-center; let mut deg=v.y.atan2(v.x).to_degrees(); if deg<0.0{deg+=360.0;}
                                if!(deg>DEAD_START && deg<DEAD_END){
                                    let nv = if deg>=START_DEG{(deg-START_DEG)/SWEEP_DEG*100.0}else{(deg+240.0)/SWEEP_DEG*100.0}.clamp(0.0,100.0);
                                    if (nv-tgt).abs()<70.0{ value.store(nv,Ordering::Relaxed); params.knob.set_value(nv); }
                                }
                            }
                        }
                        let painter = ui.painter_at(rect);
                        // ticks rounded
                        for i in 0..=40{
                            let t=i as f32/40.0; let rad=(START_DEG+t*SWEEP_DEG)*PI/180.0;
                            let major=i%10==0; let len=if major{16.0}else if i%5==0{11.0}else{7.0};
                            let w=if major{2.8}else{1.6}; let col=if t<=cur_t+0.001{lerp_color(t)}else{egui::Color32::from_gray(180)};
                            let p1=angle_to_pos(center,tick_r,rad); let p2=angle_to_pos(center,tick_r+len,rad);
                            rounded_line(&painter,p1,p2,w,col);
                        }
                        // tracks with round caps
                        let grey=egui::Color32::from_rgb(210,214,220);
                        let full_pts: Vec<_> = (0..=64).map(|i|{let t=i as f32/64.0; angle_to_pos(center,radius,start_rad+t*sweep_rad)}).collect();
                        painter.add(egui::Shape::Path(egui::epaint::PathShape{points:full_pts,closed:false,fill:egui::Color32::TRANSPARENT,stroke:egui::Stroke::new(sw,grey)}));
                        painter.circle_filled(angle_to_pos(center,radius,start_rad),sw*0.5,grey);
                        painter.circle_filled(angle_to_pos(center,radius,start_rad+sweep_rad),sw*0.5,grey);
                        if cur_t>0.001{
                            for s in 0..100{let t0=s as f32/100.0*cur_t; let t1=(s+1) as f32/100.0*cur_t; let a0=start_rad+t0*sweep_rad; let a1=start_rad+t1*sweep_rad; painter.line_segment([angle_to_pos(center,radius,a0),angle_to_pos(center,radius,a1)],egui::Stroke::new(sw,lerp_color((t0+t1)*0.5)));}
                            painter.circle_filled(angle_to_pos(center,radius,start_rad),sw*0.5,lerp_color(0.0));
                            painter.circle_filled(angle_to_pos(center,radius,cur_rad),sw*0.5,cur_col);
                        }
                        let kp=angle_to_pos(center,radius,cur_rad);
                        painter.circle_filled(kp,17.0,egui::Color32::from_black_alpha(30));
                        painter.circle_filled(kp,13.0,egui::Color32::WHITE);
                        painter.circle_stroke(kp,13.0,egui::Stroke::new(3.0,cur_col));
                        painter.text(center, egui::Align2::CENTER_CENTER, format!("{:.0}%",cur), egui::FontId::proportional(42.0), egui::Color32::BLACK);
                    });
                });
                egui_ctx.request_repaint();
            }
        )
    }
    fn process(&mut self,_b:&mut Buffer,_a:&mut AuxiliaryBuffers,_c:&mut impl IPluginApi){ }
}
impl ClapPlugin for KnobPlugin { const CLAP_ID: &'static str = "com.tayo6.knobmuse"; const CLAP_DESCRIPTION: Option<&'static str> = Some("Circular Gradient Knob"); const CLAP_MANUAL_URL: Option<&'static str> = None; const CLAP_SUPPORT_URL: Option<&'static str> = None; const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility]; }
impl Vst3Plugin for KnobPlugin { const VST3_CLASS_ID: [u8;16] = *b"knobmuseplug1234"; const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools]; }
nih_export_clap!(KnobPlugin); nih_export_vst3!(KnobPlugin);
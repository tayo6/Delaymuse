#include "PluginEditor.h"
#include <cmath>

GradientKnob::GradientKnob(juce::AudioProcessorValueTreeState& s) : apvts(s)
{
    param = dynamic_cast<juce::AudioParameterFloat*>(apvts.getParameter("knob"));
    if(param) { target = param->get(); current = target; }
    setSize(460,460);
    startTimerHz(60);
}

juce::Colour GradientKnob::lerpColor(float t)
{
    t = juce::jlimit(0.0f,1.0f,t);
    if(t <= 0.5f){ float k=t*2.0f; return juce::Colour::fromRGB((juce::uint8)(34+(250-34)*k),(juce::uint8)(197+(204-197)*k),(juce::uint8)(94+(21-94)*k)); }
    else { float k=(t-0.5f)*2.0f; return juce::Colour::fromRGB((juce::uint8)(250+(220-250)*k),(juce::uint8)(204+(38-204)*k),(juce::uint8)(21+(38-21)*k)); }
}
juce::Point<float> GradientKnob::angleToPos(juce::Point<float> c,float r,float a){ return { c.x + std::cos(a)*r, c.y + std::sin(a)*r }; }
void GradientKnob::roundedLine(juce::Graphics& g, juce::Point<float> a, juce::Point<float> b,float w,juce::Colour col)
{
    g.setColour(col); g.drawLine(juce::Line<float>(a,b), w);
    g.fillEllipse(juce::Rectangle<float>(w,w).withCentre(a));
    g.fillEllipse(juce::Rectangle<float>(w,w).withCentre(b));
}
void GradientKnob::timerCallback()
{
    float dt = 1.0f/60.0f;
    float l = 1.0f - std::exp(-dt*18.0f);
    current += (target - current)*l;
    if(param && std::abs(target - param->get()) > 0.001f) param->setValueNotifyingHost(param->convertTo0to1(target));
    repaint();
}
void GradientKnob::updateFromMouse(juce::Point<float> p)
{
    auto c = getLocalBounds().toFloat().getCentre();
    float dx = p.x - c.x, dy = p.y - c.y;
    float deg = std::atan2(dy,dx)*180.0f/3.14159265f; if(deg<0) deg+=360.0f;
    if(deg > DEAD_START && deg < DEAD_END) return;
    float nv; if(deg >= START_DEG) nv = (deg-START_DEG)/SWEEP_DEG*100.0f; else nv = (deg+240.0f)/SWEEP_DEG*100.0f;
    nv = juce::jlimit(0.0f,100.0f,nv);
    if(std::abs(nv-target) < 70.0f) target = nv;
}
void GradientKnob::mouseDown(const juce::MouseEvent& e){ updateFromMouse(e.position); }
void GradientKnob::mouseDrag(const juce::MouseEvent& e){ updateFromMouse(e.position); }

void GradientKnob::paint(juce::Graphics& g)
{
    g.fillAll(juce::Colours::white);
    auto bounds = getLocalBounds().toFloat();
    auto center = bounds.getCentre();
    float radius = 98.0f, sw = 16.0f, tickR = 128.0f;
    float startRad = START_DEG*3.14159265f/180.0f, sweepRad = SWEEP_DEG*3.14159265f/180.0f;
    float curT = current/100.0f, curRad = startRad + curT*sweepRad;
    juce::Colour curCol = lerpColor(curT);

    // ticks
    for(int i=0;i<=40;++i){ float t=i/40.0f; float rad=(START_DEG+t*SWEEP_DEG)*3.14159265f/180.0f; bool major=i%10==0; float len=major?16.0f:(i%5==0?11.0f:7.0f); float w=major?2.8f:1.6f; juce::Colour col = (t <= curT+0.001f)? lerpColor(t) : juce::Colour::fromRGB(180,180,180); auto p1=angleToPos(center,tickR,rad); auto p2=angleToPos(center,tickR+len,rad); roundedLine(g,p1,p2,w,col); }
    // grey track
    juce::Path full; full.startNewSubPath(angleToPos(center,radius,startRad));
    for(int i=1;i<=64;++i){ float t=i/64.0f; full.lineTo(angleToPos(center,radius,startRad+t*sweepRad)); }
    g.setColour(juce::Colour::fromRGB(210,214,220)); g.strokePath(full, juce::PathStrokeType(sw, juce::PathStrokeType::JointStyle::rounded, juce::PathStrokeType::EndCapStyle::rounded));
    // active gradient
    if(curT>0.001f){ for(int s=0;s<100;++s){ float t0=s/100.0f*curT, t1=(s+1)/100.0f*curT; float a0=startRad+t0*sweepRad, a1=startRad+t1*sweepRad; g.setColour(lerpColor((t0+t1)*0.5f)); g.drawLine(juce::Line<float>(angleToPos(center,radius,a0),angleToPos(center,radius,a1)), sw); } }
    auto kp = angleToPos(center,radius,curRad);
    g.setColour(juce::Colour::fromFloatRGBA(0,0,0,0.12f)); g.fillEllipse(juce::Rectangle<float>(34,34).withCentre(kp));
    g.setColour(juce::Colours::white); g.fillEllipse(juce::Rectangle<float>(26,26).withCentre(kp));
    g.setColour(curCol); g.drawEllipse(juce::Rectangle<float>(26,26).withCentre(kp),3.0f);
    g.setColour(juce::Colours::black); g.setFont(juce::Font(42.0f, juce::Font::bold)); g.drawText(juce::String((int)current)+"%", bounds, juce::Justification::centred, false);
}

KnobMuseAudioProcessorEditor::KnobMuseAudioProcessorEditor(KnobMuseAudioProcessor& p)
: AudioProcessorEditor(p), proc(p), knob(p.apvts){ addAndMakeVisible(knob); setSize(460,460); }
use anyhow::{anyhow, Result};

pub struct RecordData {
    pub width: u32,
    pub height: u32,
    pub frame_count: usize,
    pub projection_data: Vec<f32>,
    pub frame_data: Vec<u16>,
}

pub struct RecordPlayer {
    pub record_data: RecordData,
    pub stream_pos: usize,
    pub current_frame: Vec<u16>,
    pub last_frame_time: f32,
}

impl RecordPlayer {
    pub fn new(file_path: &str) -> Result<RecordPlayer> {
        let record_data =  RecordData::from_file(file_path)?;
        let current_frame = record_data.get_frame(0);

        Ok(RecordPlayer {
            record_data,
            stream_pos: 0,
            current_frame,
            last_frame_time: 0.0
        })
    }
}

impl RecordData {

    pub fn get_frame(&self, frame_num: usize) -> Vec<u16> {
        if frame_num > self.frame_count {
            eprintln!("Attempt to read frame {} from total frames {}", frame_num, self.frame_count);
            return self.get_frame(0);
        }

        let frame_size = (self.width * self.height) as usize;
        self.frame_data[frame_size*frame_num..frame_size*(frame_num+1)].to_vec()
    }

    pub fn from_file(file_path: &str) -> Result<RecordData>{
        let buf = std::fs::read(file_path)?;
        RecordData::from_buffer(&buf)
    }

    pub fn from_buffer(buf: &[u8]) -> Result<RecordData> {
        let mut ptr = 0;
        let width = i32::from_le_bytes(buf[ptr..ptr+size_of::<i32>()].try_into()?) as u32;
        ptr += size_of::<i32>();

        let height = i32::from_le_bytes(buf[ptr..ptr+size_of::<i32>()].try_into()?) as u32;
        ptr += size_of::<i32>();

        let frame_count = i32::from_le_bytes(buf[ptr..ptr+size_of::<i32>()].try_into()?) as usize;
        ptr += size_of::<i32>();

        if frame_count == 0 {
            return Err(anyhow!("No frame data in file."));
        }

        let proj_data_size = (width * height * 3) as usize;
        if buf.len() < ptr + proj_data_size * 4 {
            return Err(anyhow!("Data read from file does not match expected data."));
        }
        let projection_data: Vec<f32> = (0..proj_data_size).map(|i| { f32::from_le_bytes(buf[ptr+i*size_of::<f32>()..ptr+(i+1)*size_of::<f32>()].try_into().unwrap()) }).collect::<_>();
        ptr += proj_data_size*size_of::<f32>();

        let frame_size = (width * height) as usize;
        let frame_data_size = frame_size * frame_count;
        let frame_data: Vec<u16> = (0..frame_data_size).map(|i| { u16::from_le_bytes(buf[ptr+i*size_of::<u16>()..ptr+(i+1)*size_of::<u16>()].try_into().unwrap()) }).collect::<_>();

        Ok(RecordData {
            width,
            height,
            frame_count,
            projection_data,
            frame_data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RecordData;

    #[test]
    fn test_read() {
        let buf = include_bytes!("../../assets/recordings/record1.rdbin");
        let record_data = RecordData::from_buffer(buf).unwrap();
        assert_eq!(record_data.width, 640);
        assert_eq!(record_data.height, 480);
        assert_eq!(record_data.frame_count, 30);
    }
}
